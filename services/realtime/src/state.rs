use crate::config::Config;
use crate::pg_listener::PgListeners;
use crate::user_cache::UserCache;
use std::sync::Arc;

#[derive(Clone)]
pub struct AppState {
  pub config: Arc<Config>,
  pub pg_listeners: Arc<PgListeners>,
  pub user_cache: UserCache,
}
