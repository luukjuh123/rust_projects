use libp2p::{
  NetworkBehaviour,
  swarm::NetworkBehaviourEventProcess,
  mdns::{Mdns, MdnsEvent},
  PeerId, 
  request_response::{RequestResponse, RequestResponseCodec, ProtocolName, RequestResponseEvent, RequestResponseMessage},
};
use serde::{Deserialize, Serialize};
use futures::prelude::*;

#[derive(Debug, Clone)]
struct ChatProtocol;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ChatCodec;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ChatRequest(String);

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ChatResponse(String);

#[derive(Clone)]
pub struct ChatBehaviour {
  request_response: RequestResponse<ChatCodec>,
  mdns: Mdns,
}

impl NetworkBehaviour for ChatBehaviour {
  type ProtocolsHandler = <RequestResponse<ChatCodec> as NetworkBehaviour>::ProtocolsHandler;
  type OutEvent = ChatEvent;

  fn new_handler(&mut self) -> Self::ProtocolsHandler {
      self.request_response.new_handler()
  }

  fn addresses_of_peer(&mut self, peer_id: &PeerId) -> Vec<libp2p_core::Multiaddr> {
      self.mdns.addresses_of_peer(peer_id)
  }

  fn inject_connected(&mut self, peer_id: &PeerId) {
      self.request_response.inject_connected(peer_id)
  }

  fn inject_disconnected(&mut self, peer_id: &PeerId) {
      self.request_response.inject_disconnected(peer_id)
  }

  fn inject_event(&mut self, peer_id: PeerId, connection: libp2p::core::connection::ConnectionId, event: <Self::ProtocolsHandler as libp2p::swarm::ProtocolsHandler>::OutEvent) {
      self.request_response.inject_event(peer_id, connection, event)
  }

  fn poll(&mut self, cx: &mut std::task::Context, poll: &mut impl futures::task::Poll) -> std::task::Poll<NetworkBehaviourAction<Self::OutEvent, Self::ProtocolsHandler>> {
      while let std::task::Poll::Ready(Some(event)) = self.mdns.poll(cx, poll) {
          match event {
              MdnsEvent::Discovered(peers) => {
                  for (peer_id, _addr) in peers {
                      println!("Discovered peer: {:?}", peer_id);
                  }
              }
              MdnsEvent::Expired(peers) => {
                  for (peer_id, _addr) in peers {
                      println!("Expired peer: {:?}", peer_id);
                  }
              }
          }
      }
      self.request_response.poll(cx, poll)
  }
}

impl NetworkBehaviourEventProcess<RequestResponseEvent<ChatRequest, ChatResponse>> for ChatBehaviour {
  fn inject_event(&mut self, event: RequestResponseEvent<ChatRequest, ChatResponse>) {
      match event {
          RequestResponseEvent::Message { peer, message } => match message {
              RequestResponseMessage::Request { request, channel, .. } => {
                  println!("Received request from {:?}: {:?}", peer, request);
                  self.request_response.send_response(channel, ChatResponse("Hello from peer".to_string()));
              }
              RequestResponseMessage::Response { response, .. } => {
                  println!("Received response from {:?}: {:?}", peer, response);
              }
          },
          RequestResponseEvent::OutboundFailure { peer, error, request_id } => {
              println!("Outbound failure to {:?}: {:?}", peer, error);
          }
          RequestResponseEvent::InboundFailure { peer, error, request_id } => {
              println!("Inbound failure from {:?}: {:?}", peer, error);
          }
          RequestResponseEvent::ResponseSent { peer, request_id } => {
              println!("Response sent to {:?}", peer);
          }
      }
  }
}

#[derive(Debug, Clone)]
enum ChatEvent {
  Request(ChatRequest),
  Response(ChatResponse),
}

impl ProtocolName for ChatCodec {
  fn protocol_name(&self) -> &[u8] {
      "/chat/1.0.0".as_bytes()
  }
}

impl RequestResponseCodec for ChatCodec {
  type Protocol = ChatCodec;
  type Request = ChatRequest;
  type Response = ChatResponse;

  fn read_request<T>(&self, _: &Self::Protocol, io: &mut T) -> Result<Self::Request, std::io::Error>
  where
      T: AsyncRead + Unpin + Send,
  {
      let mut buf = Vec::new();
      futures::executor::block_on(async {
          io.read_to_end(&mut buf).await?;
          let request: ChatRequest = serde_json::from_slice(&buf)?;
          Ok(request)
      })
  }

  fn read_response<T>(&self, _: &Self::Protocol, io: &mut T) -> Result<Self::Response, std::io::Error>
  where
      T: AsyncRead + Unpin + Send,
  {
      let mut buf = Vec::new();
      futures::executor::block_on(async {
          io.read_to_end(&mut buf).await?;
          let response: ChatResponse = serde_json::from_slice(&buf)?;
          Ok(response)
      })
  }

  fn write_request<T>(&self, _: &Self::Protocol, io: &mut T, ChatRequest(data): ChatRequest) -> std::io::Result<()>
  where
      T: AsyncWrite + Unpin + Send,
  {
      let bytes = serde_json::to_vec(&ChatRequest(data))?;
      futures::executor::block_on(async {
          io.write_all(&bytes).await?;
          io.close().await?;
          Ok(())
      })
  }

  fn write_response<T>(&self, _: &Self::Protocol, io: &mut T, ChatResponse(data): ChatResponse) -> std::io::Result<()>
  where
      T: AsyncWrite + Unpin + Send,
  {
      let bytes = serde_json::to_vec(&ChatResponse(data))?;
      futures::executor::block_on(async {
          io.write_all(&bytes).await?;
          io.close().await?;
          Ok(())
      })
  }
}
