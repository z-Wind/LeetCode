use std::collections::VecDeque;
struct MyStack {
    queue:VecDeque<i32>
}


/** 
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MyStack {

    fn new() -> Self {
        Self{
            queue:VecDeque::new(),
        }
    }
    
    fn push(&mut self, x: i32) {
        let len = self.queue.len();
        self.queue.push_back(x);
        for _ in (0..len){
            let x = self.queue.pop_front().unwrap();
            self.queue.push_back(x);
        }
    }
    
    fn pop(&mut self) -> i32 {
        self.queue.pop_front().unwrap()
    }
    
    fn top(&self) -> i32 {
        *self.queue.front().unwrap()
    }
    
    fn empty(&self) -> bool {
        self.queue.is_empty()
    }
}

/**
 * Your MyStack object will be instantiated and called as such:
 * let obj = MyStack::new();
 * obj.push(x);
 * let ret_2: i32 = obj.pop();
 * let ret_3: i32 = obj.top();
 * let ret_4: bool = obj.empty();
 */