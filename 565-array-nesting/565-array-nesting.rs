impl Solution {
    pub fn array_nesting(mut nums: Vec<i32>) -> i32 {
        let n = nums.len();
        let mut ans = 0;
        for i in 0..n {
            let mut idx = i;
            let mut count = 0;
            while nums[idx] != -1 {
                // print!("{},", idx);
                count += 1;
                let temp = nums[idx];
                nums[idx] = -1;
                idx = temp as usize;
            }
            // println!("\n------------");
            
            ans = ans.max(count);
        }
        
        ans
    }
}