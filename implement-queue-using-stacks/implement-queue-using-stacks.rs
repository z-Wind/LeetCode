use std::collections::VecDeque;
struct MyQueue {
    queue:VecDeque<i32>,
}


/** 
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MyQueue {

    fn new() -> Self {
        Self{
            queue:VecDeque::new(),
        }
    }
    
    fn push(&mut self, x: i32) {
        self.queue.push_back(x);
    }
    
    fn pop(&mut self) -> i32 {
        self.queue.pop_front().unwrap()
    }
    
    fn peek(&self) -> i32 {
        *self.queue.front().unwrap()
    }
    
    fn empty(&self) -> bool {
        self.queue.is_empty()
    }
}

/**
 * Your MyQueue object will be instantiated and called as such:
 * let obj = MyQueue::new();
 * obj.push(x);
 * let ret_2: i32 = obj.pop();
 * let ret_3: i32 = obj.peek();
 * let ret_4: bool = obj.empty();
 */