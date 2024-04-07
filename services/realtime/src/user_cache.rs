use app_error::AppError;
use dashmap::DashMap;
use database::user::{select_all_uid_uuid, select_uid_from_uuid};
use futures_util::StreamExt;
use sqlx::PgPool;
use std::sync::Arc;
use uuid::Uuid;

pub struct AuthenticateUser {
  pub uid: i64,
}

pub const EXPIRED_DURATION_DAYS: i64 = 30;

#[derive(Clone)]
pub struct UserCache {
  pool: PgPool,
  users: Arc<DashMap<Uuid, AuthenticateUser>>,
}

impl UserCache {
  /// Load all users from database when initializing the cache.
  pub async fn new(pool: PgPool) -> Self {
    let users = {
      let users = DashMap::new();
      let mut stream = select_all_uid_uuid(&pool);
      while let Some(Ok(af_user_id)) = stream.next().await {
        users.insert(
          af_user_id.uuid,
          AuthenticateUser {
            uid: af_user_id.uid,
          },
        );
      }
      users
    };

    Self {
      pool,
      users: Arc::new(users),
    }
  }

  /// Get the user's uid from the cache or the database.
  pub async fn get_user_uid(&self, uuid: &Uuid) -> Result<i64, AppError> {
    if let Some(entry) = self.users.get(uuid) {
      return Ok(entry.value().uid);
    }

    // If the user is not found in the cache, query the database.
    let uid = select_uid_from_uuid(&self.pool, uuid).await?;
    self.users.insert(*uuid, AuthenticateUser { uid });
    Ok(uid)
  }
}
