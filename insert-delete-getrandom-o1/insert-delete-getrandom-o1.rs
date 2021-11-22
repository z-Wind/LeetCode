use std::collections::HashMap;
use rand::{thread_rng, Rng};
struct RandomizedSet {
    map:HashMap<i32,usize>,
    vec:Vec<i32>,
}


/** 
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl RandomizedSet {

    fn new() -> Self {
        Self{
            map:HashMap::new(),
            vec:Vec::new(),
        }
    }
    
    fn insert(&mut self, val: i32) -> bool {
        if self.map.contains_key(&val){
            return false;
        }
        self.vec.push(val);
        self.map.insert(val,self.vec.len()-1);
        true
    }
    
    fn remove(&mut self, val: i32) -> bool {
        match self.map.remove(&val){
            None => false,
            Some(i) => {
                self.vec.swap_remove(i);
                if i == self.vec.len(){
                    return true;
                }
                self.map.insert(self.vec[i], i);
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
 * Your RandomizedSet object will be instantiated and called as such:
 * let obj = RandomizedSet::new();
 * let ret_1: bool = obj.insert(val);
 * let ret_2: bool = obj.remove(val);
 * let ret_3: i32 = obj.get_random();
 */