use actix_web_actors::ws;
use futures_util::{future, SinkExt, StreamExt, TryStreamExt};
use serde_json::json;
use tokio::net::TcpStream;
use tokio_tungstenite::tungstenite::client::IntoClientRequest;
use tokio_tungstenite::tungstenite::handshake::client::Response;
use tokio_tungstenite::tungstenite::Error;
use tokio_tungstenite::tungstenite::Error::Http;
use tokio_tungstenite::{MaybeTlsStream, WebSocketStream};

#[tokio::test]
pub async fn test_realtime_client() {
  let url = "ws://0.0.0.0:8001/ws/v1?authorization=a&client-version=1.0.0&device-id=1&connect-at=1";
  let mut request = url.into_client_request().unwrap();

  let result = tokio_tungstenite::connect_async(request).await;
  match result {
    Ok((stream, response)) => {
      // let ws_stream = WebSocketStream::from_raw_socket(stream, ws::MessageCodec::new());
      // let (mut write, read) = ws_stream.split();
      // write.send(ws::Message::Text(json!({"type": "ping"}).to_string())).await.unwrap();
      // let msg = read.try_next().await.unwrap().unwrap();
      // assert_eq!(msg, ws::Message::Text(json!({"type": "pong"}).to_string()));
    }
    Err(e) => {
      match e {
        Error::Http(response) => {
          println!("response: {:?}", String::from_utf8(response.into_body().unwrap()));
        }
        _ => {
          panic!("unexpected error: {:?}", e);
        }
      }
    }
  }
  
  // let (mut write, read) = ws_stream.split();
  // write.send()
}
