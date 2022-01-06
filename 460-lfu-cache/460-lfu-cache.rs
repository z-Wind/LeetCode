// https://github.com/z-Wind/LeetCode/blob/main/lru-cache/lru-cache.rs

use std::collections::BTreeMap;
use std::collections::HashMap;
struct LFUCache {
    map: HashMap<i32, i32>,
    capacity: usize,
    key_to_count_time: HashMap<i32, (usize, usize)>,
    count_time_to_key: BTreeMap<(usize, usize), i32>,
    time: usize,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl LFUCache {
    fn new(capacity: i32) -> Self {
        let capacity = capacity as usize;
        Self {
            map: HashMap::with_capacity(capacity),
            capacity,
            key_to_count_time: HashMap::with_capacity(capacity),
            count_time_to_key: BTreeMap::new(),
            time: 0,
        }
    }

    fn get(&mut self, key: i32) -> i32 {
        if self.capacity == 0{
            return -1;
        }
        match self.map.get(&key) {
            None => -1,
            Some(&val) => {
                let (count, time) = self.key_to_count_time.get(&key).cloned().unwrap();
                self.count_time_to_key.remove(&(count, time));

                self.key_to_count_time.insert(key, (count + 1, self.time));
                self.count_time_to_key.insert((count + 1, self.time), key);
                self.time += 1;
                val
            }
        }
    }

    fn put(&mut self, key: i32, value: i32) {
        if self.capacity == 0{
            return;
        }
        let count = match self.map.get(&key) {
            None => {
                if self.map.len() == self.capacity {
                    let remove_key = self.count_time_to_key.values().next().unwrap();
                    self.map.remove(remove_key);

                    let (count, time) = self.key_to_count_time.get(remove_key).unwrap().clone();
                    self.key_to_count_time.remove(remove_key);
                    self.count_time_to_key.remove(&(count, time));
                }
                0
            }
            Some(_) => {
                let (count, time) = self.key_to_count_time.get(&key).unwrap();
                self.count_time_to_key.remove(&(*count, *time));
                *count
            }
        };

        self.map.insert(key, value);
        self.key_to_count_time.insert(key, (count + 1, self.time));
        self.count_time_to_key.insert((count + 1, self.time), key);
        self.time += 1;
        // println!("{:?} {:?} {:?}",self.map, self.key_to_count_time, self.count_time_to_key);
    }
}


/**
 * Your LFUCache object will be instantiated and called as such:
 * let obj = LFUCache::new(capacity);
 * let ret_1: i32 = obj.get(key);
 * obj.put(key, value);
 */