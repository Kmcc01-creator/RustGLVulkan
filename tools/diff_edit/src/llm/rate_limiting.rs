// src/llm/rate_limiting.rs
use tokio::sync::Semaphore;
use std::sync::Arc;

pub struct RateLimiter {
    semaphore: Arc<Semaphore>,
    window_ms: u64,
}

impl RateLimiter {
    pub fn new(max_requests: usize, window_ms: u64) -> Self {
        Self {
            semaphore: Arc::new(Semaphore::new(max_requests)),
            window_ms,
        }
    }

    pub async fn acquire(&self) -> RateLimitGuard {
        let permit = self.semaphore.acquire().await.unwrap();
        RateLimitGuard {
            _permit: permit,
            limiter: self.clone(),
        }
    }
}
