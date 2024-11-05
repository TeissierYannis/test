use std::collections::HashMap;
use std::time::{SystemTime, UNIX_EPOCH};

pub struct Whitelist {
    entries: HashMap<String, u64>,
    size: usize,
}

impl Whitelist {
    pub fn new(size: usize) -> Self {
        Self { entries: HashMap::with_capacity(size), size }
    }

    pub fn add(&mut self, src: &str, dst: &str, duration: u64) {
        let key = format!("{} {}", src, dst);
        let eol = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs() + duration;
        self.entries.insert(key, eol);
    }

    pub fn remove(&mut self, src: &str, dst: &str) {
        let key = format!("{} {}", src, dst);
        self.entries.remove(&key);
    }
}
