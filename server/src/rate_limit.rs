use std::collections::HashMap;
use std::net::IpAddr;
use std::time::{Duration, Instant};

const MAX_REQUESTS: usize = 100;
const WINDOW: Duration = Duration::from_secs(3600);

pub struct RateLimiter {
    requests: HashMap<IpAddr, Vec<Instant>>,
}

impl RateLimiter {
    pub fn new() -> Self {
        Self {
            requests: HashMap::new(),
        }
    }

    pub fn check(&mut self, ip: IpAddr) -> bool {
        let now = Instant::now();
        let entries = self.requests.entry(ip).or_default();
        entries.retain(|t| now.duration_since(*t) < WINDOW);
        if entries.len() >= MAX_REQUESTS {
            return false;
        }
        entries.push(now);
        true
    }

    pub fn cleanup(&mut self) {
        let now = Instant::now();
        self.requests.retain(|_, v| {
            v.retain(|t| now.duration_since(*t) < WINDOW);
            !v.is_empty()
        });
    }
}
