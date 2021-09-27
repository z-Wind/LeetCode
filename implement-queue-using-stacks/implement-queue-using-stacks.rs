struct MyQueue {
    stack:Vec<i32>,
    queue:Vec<i32>,
    front:i32,
}


/** 
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MyQueue {

    fn new() -> Self {
        Self{
            stack:Vec::new(),
            queue:Vec::new(),
            front:0,
        }
    }
    
    fn push(&mut self, x: i32) {
        if self.stack.is_empty(){
            self.front = x;
        }
        self.stack.push(x);
    }
    
    fn pop(&mut self) -> i32 {
        if self.queue.is_empty(){
            while let Some(x) = self.stack.pop(){
                self.queue.push(x);
            }
        }
        self.queue.pop().unwrap()
    }
    
    fn peek(&self) -> i32 {
        if self.queue.is_empty(){
            return self.front;
        }
        *self.queue.last().unwrap()
    }
    
    fn empty(&self) -> bool {
        self.stack.is_empty() && self.queue.is_empty()
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