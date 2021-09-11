use std::collections::HashMap;
use std::collections::VecDeque;
struct LRUCache {
    map: HashMap<i32,i32>,
    capacity: usize,
    deque: VecDeque<i32>,
}


/** 
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl LRUCache {

    fn new(capacity: i32) -> Self {
        let capacity = capacity as usize;
        LRUCache{
            map: HashMap::with_capacity(capacity),
            capacity,
            deque: VecDeque::with_capacity(capacity),
        }
    }
    
    fn get(&mut self, key: i32) -> i32 {
        match self.map.get(&key){
            None => -1,
            Some(&val) => {
                let i = self.deque.iter().position(|&x| x == key).unwrap();
                self.deque.remove(i);
                self.deque.push_back(key);
                val
            },
        }
    }
    
    fn put(&mut self, key: i32, value: i32) {
        match self.map.get(&key){
            None => {
                if self.map.len() == self.capacity{
                    let remove_key = self.deque.pop_front().unwrap();
                    self.map.remove(&remove_key);
                }
            },
            Some(_) => {
                let i = self.deque.iter().position(|&x| x == key).unwrap();
                self.deque.remove(i);
            },
        }
        self.deque.push_back(key);
        self.map.insert(key,value);
    }
}

/**
 * Your LRUCache object will be instantiated and called as such:
 * let obj = LRUCache::new(capacity);
 * let ret_1: i32 = obj.get(key);
 * obj.put(key, value);
 */