// 1 -> 2 (sum(..)+1)
// 1,2 -> 4 (sum(..)+1)
// 1,2,4 -> 8 (sum(..)+1)
// 1,2,4,7 -> 15 (sum(..)+1)
// 1,2,3 -> 7 (sum(..)+1)
use std::cmp::Ordering;
impl Solution {
    pub fn min_patches(nums: Vec<i32>, n: i32) -> i32 {
        let n = n as i64;
        let mut miss:i64 = 1;
        let mut count = 0;
        let mut i = 0;
        while i < nums.len() && miss <= n{
            let num = nums[i] as i64;
            match miss.cmp(&num){
                Ordering::Greater | Ordering::Equal => {
                    i+=1;
                    miss += num;
                },
                Ordering::Less => {
                    count += 1;
                    // miss += miss;
                    miss <<= 1;
                },
            }
        }
        while miss <= n{
            // println!("cur:{}, count:{}", cur, count);
            count += 1;
            // miss = sum(..miss) + miss + 1
            // sum(..miss) = miss - 1
            // miss = miss * 2
            // 1 2 4 => 8
            // 1 2 3 4 11 => 22
            miss <<= 1;
        }
        
        count
    }
}