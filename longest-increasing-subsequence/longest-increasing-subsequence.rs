// dp[i] = 1 + max(dp[j]) j=i+1..n if nums[j] > nums[i]
impl Solution {
    pub fn length_of_lis(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        let mut dp = vec![1;n];
        let mut ans = 1;
        for i in (0..n-1).rev(){
            let mut val = 0;
            for j in (i+1..n){
                if nums[j] > nums[i] {
                     val = val.max(dp[j]);
                }
            }
            dp[i] += val;
            ans = ans.max(dp[i]);
        }
        // println!("{:?}", dp);
        ans
    }
}
