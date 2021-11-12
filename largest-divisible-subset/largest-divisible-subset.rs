// dp[i] = if nums[i+1] % nums[i] => nums[i], dp[i+1]

impl Solution {
    pub fn largest_divisible_subset(mut nums: Vec<i32>) -> Vec<i32> {
        let n = nums.len();
        nums.sort();

        let mut ans = n - 1;
        let mut dp: Vec<Vec<i32>> = vec![vec![]; n];
        for i in (0..n).rev() {
            dp[i].push(nums[i]);
            let mut max_j: Option<usize> = None;
            for j in i + 1..n {
                if nums[j] % nums[i] == 0
                    && (max_j.is_none() || dp[j].len() > dp[max_j.unwrap()].len())
                {
                    max_j = Some(j);
                }
            }
            if let Some(j) = max_j {
                let mut temp = dp[j].clone();
                dp[i].append(&mut temp);
            }
            if dp[ans].len() < dp[i].len() {
                ans = i
            }
        }
        // println!("{:?}", dp);
        dp[ans].clone()
    }
}
