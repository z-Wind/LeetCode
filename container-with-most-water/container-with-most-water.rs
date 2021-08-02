use std::cmp::{min,max};

impl Solution {
    pub fn max_area(height: Vec<i32>) -> i32 {
        let mut maxarea:i32 = 0; 
        let mut l:usize = 0;
        let mut r:usize = height.len() - 1;
        while (l < r) {
            maxarea = max(maxarea, min(height[l], height[r]) * (r - l) as i32);
            if (height[l] < height[r]) {
                l+=1;
            }
            else{
                r-=1;
            }
        }
        return maxarea;
    }
}