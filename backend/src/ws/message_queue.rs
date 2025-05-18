use crate::models::websocket::WebSocketMessage;
use std::collections::VecDeque;
use std::sync::Arc;
use thiserror::Error;
use tokio::sync::{Mutex, mpsc};
use tracing::{error, warn};

const MAX_RETRIES: u32 = 3;
const RETRY_DELAY_MS: u64 = 1000;

#[derive(Error, Debug)]
pub enum MessageError {
    #[error("消息发送失败: {0}")]
    SendError(#[from] mpsc::error::SendError<WebSocketMessage>),
    #[error("消息处理失败，重试次数耗尽")]
    RetryExhausted,
}

#[derive(Debug, Clone)]
pub struct MessageQueue {
    queue: Arc<Mutex<VecDeque<WebSocketMessage>>>,
    tx: mpsc::Sender<WebSocketMessage>,
}

impl MessageQueue {
    pub fn new() -> (Self, mpsc::Receiver<WebSocketMessage>) {
        let (tx, rx) = mpsc::channel(1000);
        (
            Self {
                queue: Arc::new(Mutex::new(VecDeque::new())),
                tx,
            },
            rx,
        )
    }

    pub async fn enqueue(
        &self,
        message: WebSocketMessage,
    ) -> Result<(), mpsc::error::SendError<WebSocketMessage>> {
        self.tx.send(message).await
    }

    pub async fn process_message(
        &self,
        message: WebSocketMessage,
    ) -> Result<(), mpsc::error::SendError<WebSocketMessage>> {
        let mut retry_count = 0;

        while retry_count < MAX_RETRIES {
            match self.tx.send(message.clone()).await {
                Ok(_) => return Ok(()),
                Err(e) => {
                    if retry_count < MAX_RETRIES - 1 {
                        warn!("消息发送失败，将在 {}ms 后重试: {:?}", RETRY_DELAY_MS, e);
                        tokio::time::sleep(tokio::time::Duration::from_millis(RETRY_DELAY_MS))
                            .await;
                        retry_count += 1;
                    } else {
                        error!("消息发送失败，已达到最大重试次数: {:?}", e);
                        return Err(e);
                    }
                }
            }
        }

        Err(mpsc::error::SendError(message))
    }

    pub async fn get_queue_size(&self) -> usize {
        self.queue.lock().await.len()
    }

    pub async fn clear_queue(&self) {
        self.queue.lock().await.clear();
    }
}
