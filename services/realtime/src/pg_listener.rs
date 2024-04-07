use anyhow::Error;
use database::pg_row::AFUserNotification;
use serde::de::DeserializeOwned;
use sqlx::postgres::PgListener;
use sqlx::PgPool;
use tokio::sync::broadcast;
use tracing::{error, trace};

pub type UserListener = PostgresDBListener<AFUserNotification>;
pub struct PgListeners {
  user_listener: UserListener,
}

impl PgListeners {
  pub async fn new(pg_pool: &PgPool) -> Result<Self, Error> {
    let user_listener = UserListener::new(pg_pool, "af_user_channel").await?;

    Ok(Self { user_listener })
  }

  pub fn subscribe_user_change(&self, uid: i64) -> tokio::sync::mpsc::Receiver<AFUserNotification> {
    let (tx, rx) = tokio::sync::mpsc::channel(100);
    let mut user_notify = self.user_listener.notify.subscribe();
    tokio::spawn(async move {
      while let Ok(notification) = user_notify.recv().await {
        if let Some(row) = notification.payload.as_ref() {
          if row.uid == uid {
            let _ = tx.send(notification).await;
          }
        }
      }
    });
    rx
  }
}

pub struct PostgresDBListener<T: Clone> {
  notify: broadcast::Sender<T>,
}

impl<T> PostgresDBListener<T>
where
  T: Clone + DeserializeOwned + Send + 'static,
{
  pub async fn new(pg_pool: &PgPool, channel: &str) -> Result<Self, Error> {
    let mut listener = PgListener::connect_with(pg_pool).await?;
    // TODO(nathan): using listen_all
    listener.listen(channel).await?;

    let (tx, _) = broadcast::channel(1000);
    let notify = tx.clone();
    tokio::spawn(async move {
      while let Ok(notification) = listener.recv().await {
        trace!("Received notification: {}", notification.payload());
        match serde_json::from_str::<T>(notification.payload()) {
          Ok(change) => {
            let _ = tx.send(change);
          },
          Err(err) => {
            error!(
              "Failed to deserialize change: {:?}, payload: {}",
              err,
              notification.payload()
            );
          },
        }
      }
    });
    Ok(Self { notify })
  }
}
