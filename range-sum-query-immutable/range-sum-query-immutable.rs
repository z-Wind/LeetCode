struct NumArray {
    sums:Vec<i32>,
}


/** 
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl NumArray {

    fn new(nums: Vec<i32>) -> Self {
        let sums:Vec<i32> = nums.iter().scan(0, |sum, &x| {
            *sum+=x;
            Some(*sum)
        }).collect();
        // println!("{:?}", sums);
        Self{
            sums,
        }
    }
    
    fn sum_range(&self, left: i32, right: i32) -> i32 {
        if left == 0{
            return self.sums[right as usize];
        }
        self.sums[right as usize] - self.sums[left as usize -1]
    }
}

/**
 * Your NumArray object will be instantiated and called as such:
 * let obj = NumArray::new(nums);
 * let ret_1: i32 = obj.sum_range(left, right);
 */