use libp2p::{
    identity,
    swarm::Swarm,
    PeerId,
    mdns::{Mdns, MdnsConfig},
};
use futures::prelude::*;
use tokio::io::{AsyncBufReadExt, BufReader};

mod behaviour;
use behaviour::{ChatBehaviour, ChatRequest, ChatResponse, ChatCodec};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Generate a random keypair for the local peer.
    let local_key = identity::Keypair::generate_ed25519();
    let local_peer_id = PeerId::from(local_key.public());

    println!("Local peer id: {:?}", local_peer_id);

    // Create a Swarm to manage peers and events.
    let transport = libp2p::development_transport(local_key.clone()).await?;
    let mdns = Mdns::new(MdnsConfig::default()).await?;
    let request_response = libp2p::request_response::RequestResponse::new(
        ChatCodec,
        std::iter::once((ChatCodec, libp2p::request_response::ProtocolSupport::Full)),
        libp2p::request_response::Config::default(),
    );

    let mut swarm = Swarm::new(transport, ChatBehaviour { request_response, mdns }, local_peer_id);

    // Read lines from stdin
    let mut stdin = BufReader::new(tokio::io::stdin()).lines();

    // Start the event loop.
    loop {
        tokio::select! {
            line = stdin.next_line() => {
                if let Some(line) = line? {
                    for peer_id in swarm.behaviour().request_response.connected_peers() {
                        swarm.behaviour_mut().request_response.send_request(&peer_id, ChatRequest(line.clone()));
                    }
                }
            },
            event = swarm.next() => {
                match event {
                    Some(SwarmEvent::Behaviour(behaviour::ChatEvent::Request(request))) => {
                        println!("Received request: {:?}", request);
                    },
                    Some(SwarmEvent::Behaviour(behaviour::ChatEvent::Response(response))) => {
                        println!("Received response: {:?}", response);
                    },
                    _ => {},
                }
            }
        }
    }
}
