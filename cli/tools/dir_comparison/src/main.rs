use std::collections::HashMap;
use std::fs;
use std::path::Path;
use log::{error, info, warn};
use walkdir::WalkDir;
use pavao::{SmbClient, SmbCredentials, SmbOptions, SmbDirent};
use dotenv::dotenv;
use std::env;

fn list_smb_dir_pavao(
    client: &SmbClient,
    remote_path: &str,
    prefix: &str,
    files: &mut HashMap<String, u64>,
) {
    let entries: Vec<SmbDirent> = match client.list_dir(remote_path) {
        Ok(entries) => entries,
        Err(e) => {
            warn!("Failed to list directory {}: {}", remote_path, e);
            return;
        }
    };

    for entry in entries {
        if entry.name == "." || entry.name == ".." {
            continue;
        }
        let rel_path = if prefix.is_empty() {
            entry.name.clone()
        } else {
            format!("{}/{}", prefix, entry.name)
        };

        if entry.is_dir {
            let new_remote = format!("{}/{}", remote_path, entry.name);
            list_smb_dir_pavao(client, &new_remote, &rel_path, files);
        } else {
            files.insert(rel_path, entry.size);
        }
    }
}

fn get_all_smb_files_from_pavao(
    smb_url: &str,
    username: &str,
    password: &str,
) -> HashMap<String, u64> {
    let mut files = HashMap::new();

    let creds = SmbCredentials {
        username: username.to_string(),
        password: password.to_string(),
        domain: None,
    };

    let opts = SmbOptions {
        url: smb_url.to_string(),
        ..Default::default()
    };

    let client = match SmbClient::new(creds, opts) {
        Ok(client) => client,
        Err(e) => {
            error!("Failed to create SMB client: {}", e);
            return files;
        }
    };

    list_smb_dir_pavao(&client, "SSD_backup", "", &mut files);
    files
}

fn get_all_local_files<P: AsRef<Path>>(directory: P) -> HashMap<String, u64> {
    let mut file_map = HashMap::new();

    for entry in WalkDir::new(&directory).into_iter().filter_map(Result::ok) {
        if entry.file_type().is_file() {
            let full_path = entry.path();
            let size = match fs::metadata(full_path) {
                Ok(metadata) => metadata.len(),
                Err(e) => {
                    warn!("Could not get metadata for {:?}: {}", full_path, e);
                    continue;
                }
            };

            let relative = match full_path.strip_prefix(&directory) {
                Ok(rel) => rel.to_string_lossy().into_owned(),
                Err(_) => continue,
            };

            file_map.insert(relative, size);
        }
    }
    file_map
}

fn check_transfer(
    local_files: &HashMap<String, u64>,
    smb_files: &HashMap<String, u64>,
) {
    let mut missing_files = vec![];
    let mut size_mismatches = vec![];
    let mut successful_transfers = vec![];

    info!("🔍 Starting file transfer verification...");

    for (file_path, &local_size) in local_files {
        match smb_files.get(file_path) {
            None => {
                warn!("❌ MISSING: {}", file_path);
                missing_files.push(file_path);
            }
            Some(&smb_size) => {
                if smb_size != local_size {
                    warn!(
                        "⚠️ SIZE MISMATCH: {} (Local: {} bytes, SMB: {} bytes)",
                        file_path, local_size, smb_size
                    );
                    size_mismatches.push(file_path);
                } else {
                    info!("✅ SUCCESS: {}", file_path);
                    successful_transfers.push(file_path);
                }
            }
        }
    }

    info!("🎯 Transfer Summary:");
    info!("✅ {} files successfully transferred.", successful_transfers.len());
    if !missing_files.is_empty() {
        warn!("❌ {} files are missing.", missing_files.len());
    }
    if !size_mismatches.is_empty() {
        warn!("⚠️ {} files have mismatched sizes.", size_mismatches.len());
    }
    if missing_files.is_empty() && size_mismatches.is_empty() {
        info!("🎉 All files have been successfully transferred!");
    }
}

fn main() {
    // Load environment variables from .env file
    dotenv().ok();

    // Retrieve credentials from the environment variables
    let username = env::var("SMB_USERNAME").expect("SMB_USERNAME must be set in .env");
    let password = env::var("SMB_PASSWORD").expect("SMB_PASSWORD must be set in .env");

    // Initialize logging (set RUST_LOG=info for detailed logging).
    env_logger::init();

    let local_dir = "/mnt/external";
    info!("🔍 Fetching local files from {}", local_dir);
    let local_files = get_all_local_files(local_dir);
    info!("Found {} local files.", local_files.len());

    let smb_server = "omv.local";
    let smb_share = "mammoth";
    // Construct the SMB URL to point to the appropriate share.
    let smb_url = format!("smb://{}/{}/SSD_backup", smb_server, smb_share);
    info!("🔍 Fetching SMB files from {}", smb_url);
    let smb_files = get_all_smb_files_from_pavao(&smb_url, &username, &password);
    info!("Found {} files on the SMB share.", smb_files.len());

    info!("🔍 Comparing local files with SMB files...");
    check_transfer(&local_files, &smb_files);

    info!("📜 Transfer verification complete. See log output for details.");
}
