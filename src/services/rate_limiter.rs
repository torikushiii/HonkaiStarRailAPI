use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::Mutex;
use std::time::{Duration, Instant};

pub struct RateLimiter {
    requests: Arc<Mutex<HashMap<String, (Instant, u32)>>>,
    max_requests: u32,
    window: Duration,
}

impl RateLimiter {
    pub fn new(max_requests: u32, window_seconds: u64) -> Self {
        Self {
            requests: Arc::new(Mutex::new(HashMap::new())),
            max_requests,
            window: Duration::from_secs(window_seconds),
        }
    }

    pub async fn check_rate_limit(&self, ip: &str) -> bool {
        let mut requests = self.requests.lock().await;
        let now = Instant::now();
        
        // Clean up old entries
        requests.retain(|_, (timestamp, _)| now.duration_since(*timestamp) < self.window);
        
        if let Some((timestamp, count)) = requests.get_mut(ip) {
            if now.duration_since(*timestamp) < self.window {
                if *count >= self.max_requests {
                    return false;
                }
                *count += 1;
            } else {
                *timestamp = now;
                *count = 1;
            }
        } else {
            requests.insert(ip.to_string(), (now, 1));
        }
        
        true
    }
} 