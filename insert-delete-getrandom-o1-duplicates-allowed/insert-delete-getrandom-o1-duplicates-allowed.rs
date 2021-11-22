use std::collections::{HashMap,HashSet};
use rand::{thread_rng, Rng};
struct RandomizedCollection {
    map:HashMap<i32,HashSet<usize>>,
    vec:Vec<i32>,
}


/** 
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl RandomizedCollection {

    fn new() -> Self {
        Self{
            map:HashMap::new(),
            vec:Vec::new(),
        }
    }
    
    fn insert(&mut self, val: i32) -> bool {
        let exist = self.map.contains_key(&val);
        self.map.entry(val).or_default().insert(self.vec.len());
        self.vec.push(val);
        !exist
    }
    
    fn remove(&mut self, val: i32) -> bool {
        match self.map.remove(&val){
            None => false,
            Some(mut set) => {
                let i = set.iter().next().cloned().unwrap();
                self.vec.swap_remove(i);
                set.remove(&i);
                if !set.is_empty(){
                    self.map.insert(val,set);
                }
                if i == self.vec.len(){
                    return true;
                }
                let mut set = self.map.get_mut(&self.vec[i]).unwrap();
                set.remove(&self.vec.len());
                set.insert(i);
                true
            },
        }
    }
    
    fn get_random(&self) -> i32 {
        let mut rng = thread_rng();
        let i: usize = rng.gen_range(0, self.vec.len());
        self.vec[i]
    }
}

/**
 * Your RandomizedCollection object will be instantiated and called as such:
 * let obj = RandomizedCollection::new();
 * let ret_1: bool = obj.insert(val);
 * let ret_2: bool = obj.remove(val);
 * let ret_3: i32 = obj.get_random();
 */