use std::collections::HashMap;  

pub struct KvStore {
    map: HashMap<String, String>,
}

impl KvStore {
    pub fn new() -> KvStore {
        return KvStore {map: HashMap::new(),};
    }

    pub fn set(&mut self, key: String, value: String) {
        self.map.insert(key, value);
    }

    pub fn get(&self, key: String) -> Option<String> {
        return self.map.get(&key).cloned();
    }
        
    pub fn remove(&mut self, key: String) {
        self.map.remove(&key);
    }
}
