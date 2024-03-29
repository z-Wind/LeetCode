use std::collections::{HashMap,BTreeMap,HashSet};
use std::cmp::Reverse;
#[derive(Default, Debug)]
struct AllOne {
    map:HashMap<String,i32>,
    min_map:BTreeMap<i32,HashSet<String>>,
    max_map:BTreeMap<Reverse<i32>,HashSet<String>>,
}


/** 
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl AllOne {

    fn new() -> Self {
        Default::default()
    }
    
    fn inc(&mut self, key: String) {
        let count = self.map.entry(key.clone()).or_default();
        let pre_count = *count;
        *count += 1;
        let count = *count;
        
        self.min_map.entry(count).or_default().insert(key.clone());
        if pre_count != 0{
            let mut set = self.min_map.get_mut(&pre_count).unwrap();
            set.remove(&key);    
            if set.is_empty(){
                    self.min_map.remove(&pre_count);
            }
        }
        
        self.max_map.entry(Reverse(count)).or_default().insert(key.clone());
        if pre_count != 0{
            let mut set = self.max_map.get_mut(&Reverse(pre_count)).unwrap();
            set.remove(&key);    
            if set.is_empty(){
                    self.max_map.remove(&Reverse(pre_count));
            }
        }
        
        // println!("map:{:?}", self.map);
        // println!("min:{:?}", self.min_map);
        // println!("max:{:?}", self.max_map);
    }
    
    fn dec(&mut self, key: String) {
        let val = match self.map.get_mut(&key){
            None => -1,
            Some(x) => {
                let mut set = self.min_map.get_mut(x).unwrap();
                set.remove(&key);
                if set.is_empty(){
                    self.min_map.remove(&x);
                }
                
                let mut set = self.max_map.get_mut(&Reverse(*x)).unwrap();
                set.remove(&key);
                if set.is_empty(){
                    self.max_map.remove(&Reverse(*x));
                }
                
                *x -= 1;
                *x
            },
        };
        if val == 0{
            self.map.remove(&key);
        } else {
            self.min_map.entry(val).or_default().insert(key.clone());
            self.max_map.entry(Reverse(val)).or_default().insert(key);
        }
    }
    
    fn get_max_key(&self) -> String {
        match self.max_map.values().next(){
            None => String::new(),
            Some(set) => match set.iter().next(){
                None => String::new(),
                Some(s) => s.clone(),
            }
        }
    }
    
    fn get_min_key(&self) -> String {
        match self.min_map.values().next(){
            None => String::new(),
            Some(set) => match set.iter().next(){
                None => String::new(),
                Some(s) => s.clone(),
            }
        }
    }
}

/**
 * Your AllOne object will be instantiated and called as such:
 * let obj = AllOne::new();
 * obj.inc(key);
 * obj.dec(key);
 * let ret_3: String = obj.get_max_key();
 * let ret_4: String = obj.get_min_key();
 */