impl Solution {
    pub fn find_max_length(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        let mut counts = vec![(0,0);n+1];
        
        let mut ans = 0;
        for i in 1..=n {
            counts[i] = counts[i-1];
            match nums[i-1] {
                0 => counts[i].0 += 1,
                1 => counts[i].1 += 1,
                _ => (),
            }
            // println!("{}:{:?}", i, counts[i]);
            
            let max_w = 2 * counts[i].0.min(counts[i].1);
            let mut j = i - ans - 2;
            while i - j <= max_w {
                if counts[i].0 - counts[j].0 == counts[i].1 - counts[j].1 {
                    ans = i - j;
                }
                j -= 2;
            }
        }
        
        ans as i32
    }
}
