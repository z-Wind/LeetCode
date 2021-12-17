use std::collections::{HashMap,BTreeMap,HashSet};

#[derive(Default, Debug)]
struct AllOne {
    map:HashMap<String,i32>,
    min_map:BTreeMap<i32,HashSet<String>>,
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
        *count += 1;
        self.min_map.entry(*count).or_default().insert(key.clone());
        if *count > 1{
            let count = (*count-1);
            let mut set = self.min_map.get_mut(&count).unwrap();
            set.remove(&key);    
            if set.is_empty(){
                    self.min_map.remove(&count);
            }
        }
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
                *x -= 1;
                *x
            },
        };
        if val == 0{
            self.map.remove(&key);
        } else {
            self.min_map.entry(val).or_default().insert(key);
        }
    }
    
    fn get_max_key(&self) -> String {
        match self.min_map.values().last(){
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