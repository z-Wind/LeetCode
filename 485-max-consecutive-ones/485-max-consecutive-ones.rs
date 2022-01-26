impl Solution {
    pub fn find_max_consecutive_ones(nums: Vec<i32>) -> i32 {
        let mut ans = 0;
        let mut count = 0;
        for num in nums {
            match num {
                0 => count = 0,
                1 => { 
                    count += 1;
                    ans = ans.max(count);
                },
                _ => panic!("impossible"),
            }
        }
        
        ans
    }
}