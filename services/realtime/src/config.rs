use anyhow::Context;
use secrecy::Secret;
use sqlx::postgres::{PgConnectOptions, PgSslMode};
use std::str::FromStr;

#[derive(Clone, Debug)]
pub struct Config {
  pub application: ApplicationSetting,
  pub websocket: WebsocketSetting,
  pub db_settings: DatabaseSetting,
  pub gotrue: GoTrueSetting,
}

#[derive(Clone, Debug)]
pub struct ApplicationSetting {
  pub port: u16,
  pub host: String,
}

#[derive(Clone, Debug)]
pub struct WebsocketSetting {
  pub heartbeat_interval: u8,
  pub client_timeout: u8,
}

#[derive(Clone, Debug)]
pub struct DatabaseSetting {
  pub pg_conn_opts: PgConnectOptions,
  pub require_ssl: bool,
  pub max_connections: u32,
  pub database_name: String,
}

impl DatabaseSetting {
  pub fn without_db(&self) -> PgConnectOptions {
    let ssl_mode = if self.require_ssl {
      PgSslMode::Require
    } else {
      PgSslMode::Prefer
    };
    let options = self.pg_conn_opts.clone();
    options.ssl_mode(ssl_mode)
  }

  pub fn with_db(&self) -> PgConnectOptions {
    self.without_db().database(&self.database_name)
  }
}

#[derive(serde::Deserialize, Clone, Debug)]
pub struct GoTrueSetting {
  pub jwt_secret: Secret<String>,
}

pub fn get_env_var(key: &str, default: &str) -> String {
  std::env::var(key).unwrap_or_else(|e| {
    tracing::warn!(
      "failed to read environment variable: {}, using default value: {}",
      e,
      default
    );
    default.to_owned()
  })
}

pub fn get_configuration() -> Result<Config, anyhow::Error> {
  let config = Config {
    application: ApplicationSetting {
      port: get_env_var("APPFLOWY_APPLICATION_PORT", "8001").parse()?,
      host: get_env_var("APPFLOWY_APPLICATION_HOST", "0.0.0.0"),
    },
    websocket: WebsocketSetting {
      heartbeat_interval: get_env_var("APPFLOWY_WEBSOCKET_HEARTBEAT_INTERVAL", "6").parse()?,
      client_timeout: get_env_var("APPFLOWY_WEBSOCKET_CLIENT_TIMEOUT", "60").parse()?,
    },
    db_settings: DatabaseSetting {
      pg_conn_opts: PgConnectOptions::from_str(&get_env_var(
        "APPFLOWY_DATABASE_URL",
        "postgres://postgres:password@localhost:5432/postgres",
      ))?,
      require_ssl: get_env_var("APPFLOWY_DATABASE_REQUIRE_SSL", "false")
        .parse()
        .context("fail to get APPFLOWY_DATABASE_REQUIRE_SSL")?,
      max_connections: get_env_var("APPFLOWY_DATABASE_MAX_CONNECTIONS", "40")
        .parse()
        .context("fail to get APPFLOWY_DATABASE_MAX_CONNECTIONS")?,
      database_name: get_env_var("APPFLOWY_DATABASE_NAME", "postgres"),
    },
    gotrue: GoTrueSetting {
      jwt_secret: get_env_var("APPFLOWY_GOTRUE_JWT_SECRET", "hello456").into(),
    },
  };
  Ok(config)
}
