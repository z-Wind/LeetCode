// https://leetcode.com/problems/find-median-from-data-stream/discuss/74047/JavaPython-two-heap-solution-O(log-n)-add-O(1)-find

use std::collections::BinaryHeap;
use std::cmp::Reverse;
struct MedianFinder {
    before:BinaryHeap<i32>,
    after:BinaryHeap<Reverse<i32>>,
}


/** 
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MedianFinder {

    fn new() -> Self {
        Self{
            before:BinaryHeap::new(),
            after:BinaryHeap::new(),
        }
    }
    
    fn add_num(&mut self, num: i32) {
        if self.before.len() == self.after.len(){
            self.after.push(Reverse(num));
            self.before.push(self.after.pop().unwrap().0);
        } else {
            self.before.push(num);
            self.after.push(Reverse(self.before.pop().unwrap()));
        }
    }
    
    fn find_median(&self) -> f64 {
        if self.before.len() == self.after.len(){
            (self.before.peek().unwrap() + self.after.peek().unwrap().0) as f64 / 2.0
        } else {
            *self.before.peek().unwrap() as f64
        }
    }
}

/**
 * Your MedianFinder object will be instantiated and called as such:
 * let obj = MedianFinder::new();
 * obj.add_num(num);
 * let ret_2: f64 = obj.find_median();
 */