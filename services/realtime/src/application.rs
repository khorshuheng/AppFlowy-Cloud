use crate::api::ws_scope;
use crate::config::{Config, DatabaseSetting};
use crate::pg_listener::PgListeners;
use crate::state::AppState;
use crate::user_cache::UserCache;
use actix_web::dev::Server;
use actix_web::web::Data;
use actix_web::{App, HttpServer};
use anyhow::Error;
use sqlx::postgres::PgPoolOptions;
use sqlx::PgPool;
use std::net::TcpListener;
use std::sync::Arc;
use std::time::Duration;
use actix::Actor;
use tracing::info;
use crate::remote_server::RemoteRealtimeServerActor;

pub struct Application {
  port: u16,
  server: Server,
}

impl Application {
  pub async fn build(config: Config, state: AppState) -> Result<Self, Error> {
    let address = format!("{}:{}", config.application.host, config.application.port);
    println!("Starting server at http://{}", address);
    let listener = TcpListener::bind(&address)?;
    let port = listener.local_addr().unwrap().port();
    let server = run(listener, state, config).await?;

    Ok(Self { port, server })
  }

  pub async fn run_until_stopped(self) -> Result<(), std::io::Error> {
    self.server.await
  }
}

pub async fn run(
  listener: TcpListener,
  state: AppState,
  config: Config,
) -> Result<Server, anyhow::Error> {
  let realtime_server_actor = RemoteRealtimeServerActor::new().start();
  let mut server = HttpServer::new(move || {
    App::new()
      .app_data(Data::new(state.clone()))
      .app_data(Data::new(realtime_server_actor.clone()))
      .service(ws_scope())
  });
  server = server.listen(listener)?;

  Ok(server.run())
}

pub async fn init_state(config: &Config) -> Result<AppState, Error> {
  let pg_pool = get_connection_pool(&config.db_settings).await?;

  // User cache
  let user_cache = UserCache::new(pg_pool.clone()).await;

  // Pg listeners
  info!("Setting up Pg listeners...");
  let pg_listeners = Arc::new(PgListeners::new(&pg_pool).await?);
  let app_state = AppState {
    config: Arc::new(config.clone()),
    pg_listeners,
    user_cache,
  };
  Ok(app_state)
}

async fn get_connection_pool(setting: &DatabaseSetting) -> Result<PgPool, Error> {
  info!(
    "Connecting to postgres database with setting: {:?}",
    setting
  );
  PgPoolOptions::new()
    .max_connections(setting.max_connections)
    .acquire_timeout(Duration::from_secs(10))
    .max_lifetime(Duration::from_secs(60 * 60))
    .idle_timeout(Duration::from_secs(60))
    .connect_with(setting.with_db())
    .await
    .map_err(|e| anyhow::anyhow!("Failed to connect to postgres database: {}", e))
}
