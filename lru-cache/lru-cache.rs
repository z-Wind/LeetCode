use std::collections::HashMap;
use std::collections::BTreeMap;
struct LRUCache {
    map: HashMap<i32,i32>,
    capacity: usize,
    key_to_time: HashMap<i32,usize>,
    time_to_key: BTreeMap<usize,i32>,
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
            key_to_time: HashMap::with_capacity(capacity),
            time_to_key: BTreeMap::new(),
            time: 0,
        }
    }
    
    fn get(&mut self, key: i32) -> i32 {
        match self.map.get(&key){
            None => -1,
            Some(&val) => {
                let time = self.key_to_time.get(&key).unwrap();
                self.time_to_key.remove(time);
                
                self.key_to_time.insert(key, self.time);
                self.time_to_key.insert(self.time, key);
                self.time+=1;
                val
            },
        }
    }
    
    fn put(&mut self, key: i32, value: i32) {
        match self.map.get(&key){
            None => {
                if self.map.len() == self.capacity{
                    let remove_key = self.time_to_key.values().next().unwrap();
                    self.map.remove(remove_key);

                    let time = self.key_to_time.get(remove_key).unwrap().clone();
                    self.key_to_time.remove(remove_key);
                    self.time_to_key.remove(&time);
                }
            },
            Some(_) => {
                let time = self.key_to_time.get(&key).unwrap();
                self.time_to_key.remove(time);
            },
        }
        
        self.map.insert(key,value);        
        self.key_to_time.insert(key, self.time);
        self.time_to_key.insert(self.time, key);
        self.time+=1;
        // println!("{:?} {:?} {:?}",self.map, self.key_to_time, self.time_to_key);
    }
}

/**
 * Your LRUCache object will be instantiated and called as such:
 * let obj = LRUCache::new(capacity);
 * let ret_1: i32 = obj.get(key);
 * obj.put(key, value);
 */