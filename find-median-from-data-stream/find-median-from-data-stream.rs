use std::collections::VecDeque;
struct MedianFinder {
    nums:VecDeque<i32>,
}


/** 
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MedianFinder {

    fn new() -> Self {
        Self{
            nums:VecDeque::new(),
        }
    }
    
    fn add_num(&mut self, num: i32) {
        if self.nums.is_empty() || num >= self.nums[self.nums.len()-1]{
            self.nums.push_back(num);
        } else if num <= self.nums[0] {
            self.nums.push_front(num);
        } else {
            let mut pos = 0;
            while num > self.nums[pos]{
                pos += 1;
            }
            self.nums.insert(pos, num);  
        }
        // println!("{:?}", self.nums);
    }
    
    fn find_median(&self) -> f64 {
        let i = self.nums.len()/2;
        if self.nums.len()%2 == 0{
            return (self.nums[i-1]+self.nums[i]) as f64 / 2.0;
        } else {
            return self.nums[i] as f64;
        }
    }
}

/**
 * Your MedianFinder object will be instantiated and called as such:
 * let obj = MedianFinder::new();
 * obj.add_num(num);
 * let ret_2: f64 = obj.find_median();
 */