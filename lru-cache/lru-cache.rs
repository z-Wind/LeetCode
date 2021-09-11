use std::collections::HashMap;
use std::collections::VecDeque;
struct LRUCache {
    map: HashMap<i32,i32>,
    capacity: usize,
    time_order: HashMap<i32,usize>,
    time: usize,
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
            time_order: HashMap::with_capacity(capacity),
            time: 0,
        }
    }
    
    fn get(&mut self, key: i32) -> i32 {
        match self.map.get(&key){
            None => -1,
            Some(&val) => {
                self.time_order.insert(key, self.time);
                self.time+=1;
                val
            },
        }
    }
    
    fn put(&mut self, key: i32, value: i32) {
        self.map.insert(key,value);
        if self.map.len() > self.capacity{
            let mut old_time = usize::MAX;
            let mut remove_key = 0;
            for (key, &time) in self.time_order.iter(){
                if time < old_time{
                    old_time = time;
                    remove_key = *key;
                }
            }
            self.map.remove(&remove_key);
            self.time_order.remove(&remove_key);
        }
        self.time_order.insert(key, self.time);
        self.time+=1;
        // println!("{:?} {:?}",self.map, self.time_order);
    }
}

/**
 * Your LRUCache object will be instantiated and called as such:
 * let obj = LRUCache::new(capacity);
 * let ret_1: i32 = obj.get(key);
 * obj.put(key, value);
 */