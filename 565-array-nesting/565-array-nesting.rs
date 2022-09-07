impl Solution {
    pub fn array_nesting(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        let mut visited = vec![false; n];
        let mut ans = 0;
        for i in 0..n {
            let mut idx = i;
            let mut count = 0;
            while !visited[idx] {
                // print!("{},", idx);
                count += 1;
                visited[idx] = true;
                idx = nums[idx] as usize;
            }
            // println!("\n------------");
            
            ans = ans.max(count);
        }
        
        ans
    }
}