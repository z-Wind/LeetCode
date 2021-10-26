// 1 -> 2 (sum(..)+1)
// 1,2 -> 4 (sum(..)+1)
// 1,2,4 -> 8 (sum(..)+1)
// 1,2,4,7 -> 15 (sum(..)+1)
// 1,2,3 -> 7 (sum(..)+1)
use std::cmp::Ordering;
impl Solution {
    pub fn min_patches(nums: Vec<i32>, n: i32) -> i32 {
        let n = n as i64;
        let mut cur:i64 = 1;
        let mut sum:i64 = 0;
        let mut count = 0;
        let mut i = 0;
        while i < nums.len() && cur-1 < n{
            let num = nums[i] as i64;
            // println!("i:{}=>{} ,cur:{}, sum:{}, count:{}",i, nums[i], cur, sum, count);
            match cur.cmp(&num){
                Ordering::Greater => {
                    sum += num;
                    cur += num;
                    i+=1;
                    continue;
                },
                Ordering::Equal => {
                    sum += num;
                    i+=1;
                },
                Ordering::Less => {
                    sum += cur;
                    count += 1;
                },
            }
            cur = sum + 1;
        }
        while cur-1 < n{
            // println!("cur:{}, count:{}", cur, count);
            count += 1;
            // cur = sum(..cur) + cur + 1
            // sum(..cur) = cur - 1
            // cur = cur * 2
            // 1 2 4 => 8
            // 1 2 3 4 11 => 22
            cur <<= 1;
        }
        
        count
    }
}