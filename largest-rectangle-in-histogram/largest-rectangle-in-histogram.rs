// https://leetcode.com/problems/largest-rectangle-in-histogram/discuss/28902/5ms-O(n)-Java-solution-explained-(beats-96)
use std::cmp::max;
impl Solution {
    pub fn largest_rectangle_area(heights: Vec<i32>) -> i32 {
        if (heights.len() == 0) {
            return 0;
        }
        let mut lessFromLeft:Vec<i32> = vec![0;heights.len()]; // idx of the first bar the left that is lower than current
        let mut lessFromRight:Vec<i32> = vec![0;heights.len()]; // idx of the first bar the right that is lower than current
        lessFromRight[heights.len() - 1] = heights.len() as i32;
        lessFromLeft[0] = -1;

        for i in (1..heights.len() as i32) {
            let mut p:i32 = i - 1;
            while (p >= 0 && heights[p as usize] >= heights[i as usize]) {
                p = lessFromLeft[p as usize];
            }
            lessFromLeft[i as usize] = p;
        }

        for i in (0..(heights.len()-1) as i32).rev() {
            let mut p:i32 = i + 1;
            while (p < heights.len() as i32 && heights[p as usize] >= heights[i as usize]) {
                p = lessFromRight[p as usize];
            }
            lessFromRight[i as usize] = p;
        }
        
        //println!("{:?} {:?}", lessFromLeft, lessFromRight);
        let mut maxArea = 0;
        for i in (0..heights.len()) {
            maxArea = max(maxArea, heights[i] * (lessFromRight[i] - lessFromLeft[i] - 1));
        }

        return maxArea;
    }
}