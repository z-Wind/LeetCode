struct NumArray {
    nums:Vec<i32>,
}


/** 
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl NumArray {

    fn new(nums: Vec<i32>) -> Self {
        Self{
            nums,
        }
    }
    
    fn update(&mut self, index: i32, val: i32) {
        let index = index as usize;
        self.nums[index] = val;
    }
    
    fn sum_range(&self, left: i32, right: i32) -> i32 {
        let left = left as usize;
        let right = right as usize;
        
        self.nums[left..=right].iter().sum()
    }
}

/**
 * Your NumArray object will be instantiated and called as such:
 * let obj = NumArray::new(nums);
 * obj.update(index, val);
 * let ret_2: i32 = obj.sum_range(left, right);
 */