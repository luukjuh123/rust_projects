use bytes::Buf;
use futures::StreamExt;
use std::fmt;
use std::fs::File;
use std::io::Write;
use warp::multipart::FormData;

struct CustomRejection(String);

impl fmt::Debug for CustomRejection {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "CustomRejection({})", self.0)
    }
}

impl warp::reject::Reject for CustomRejection {}

pub async fn upload_handler(form: FormData) -> Result<impl warp::Reply, warp::Rejection> {
    let mut form = form;

    while let Some(part) = form.next().await {
        match part {
            Ok(part) => {
                if let Some(filename) = part.filename() {
                    let file_path = format!("./sink/{}", filename);
                    let mut file = File::create(&file_path).expect("Failed to create file");

                    let mut stream = part.stream();
                    while let Some(chunk) = stream.next().await {
                        match chunk {
                            Ok(buf) => {
                                let data = buf.chunk();
                                file.write_all(data).expect("Failed to write to file");
                            }
                            Err(err) => {
                                eprintln!("Error processing chunk: {:?}", err);
                                return Err(warp::reject::custom(CustomRejection(
                                    "Error processing file".to_string(),
                                )));
                            }
                        }
                    }
                }
            }
            Err(err) => {
                eprintln!("Error processing form data: {:?}", err);
                return Err(warp::reject::custom(CustomRejection(
                    "Error processing form data".to_string(),
                )));
            }
        }
    }

    Ok(warp::reply::with_status(
        "File uploaded successfully.",
        warp::http::StatusCode::OK,
    ))
}
