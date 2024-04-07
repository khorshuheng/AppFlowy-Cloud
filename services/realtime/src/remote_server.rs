use crate::client::{HandlerResult, RealtimeServer};
use crate::entities::{ClientMessage, Connect, Disconnect};
use actix::{Actor, Context, Handler};
use tokio_tungstenite::connect_async;
use tracing::info;

pub struct RemoteRealtimeServerActor {
  
}

impl RemoteRealtimeServerActor {
  pub fn new() -> Self {
    Self {}
  }
}

impl Actor for RemoteRealtimeServerActor {
  type Context = Context<Self>;

  fn started(&mut self, ctx: &mut Self::Context) {
    info!("realtime server started");
    ctx.set_mailbox_capacity(3000);
  }
}

impl Handler<ClientMessage> for RemoteRealtimeServerActor {
  type Result = HandlerResult;

  fn handle(&mut self, _msg: ClientMessage, _ctx: &mut Self::Context) -> Self::Result {
    println!("handling client message");
    Box::pin(async { Ok(()) })
  }
}

impl Handler<Connect> for RemoteRealtimeServerActor {
  type Result = HandlerResult;

  fn handle(&mut self, _msg: Connect, _ctx: &mut Self::Context) -> Self::Result {
    println!("handle new connection");
    Box::pin(async { Ok(()) })
  }
}

impl Handler<Disconnect> for RemoteRealtimeServerActor {
  type Result = HandlerResult;

  fn handle(&mut self, _msg: Disconnect, _ctx: &mut Self::Context) -> Self::Result {
    println!("handle disconnect");
    Box::pin(async { Ok(()) })
  }
}

impl RealtimeServer for RemoteRealtimeServerActor {}
