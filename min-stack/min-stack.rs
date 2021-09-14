struct MinStack {
    stack:Vec<(i32,i32)>,
}


/** 
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MinStack {

    /** initialize your data structure here. */
    fn new() -> Self {
        MinStack{
            stack: Vec::new(),
        }
    }
    
    fn push(&mut self, val: i32) {
        match self.stack.last(){
            None => self.stack.push((val,val)),
            Some(x) => {
                let min = match val < x.1 {
                    true => val,
                    false => x.1,
                };
                self.stack.push((val,min));
            }
        }
        
    }
    
    fn pop(&mut self) {
        self.stack.pop();
    }
    
    fn top(&self) -> i32 {
        self.stack.last().unwrap().0
    }
    
    fn get_min(&self) -> i32 {
        self.stack.last().unwrap().1
    }
}

/**
 * Your MinStack object will be instantiated and called as such:
 * let obj = MinStack::new();
 * obj.push(val);
 * obj.pop();
 * let ret_3: i32 = obj.top();
 * let ret_4: i32 = obj.get_min();
 */