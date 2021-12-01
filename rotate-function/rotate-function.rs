// F(0) = nums[0] * 0 + nums[1] * 1 + .. nums[n-1] * n-1
// F(1) = nums[n-1] * 0 + nums[0] * 1 + .. nums[n-2] * n-1
//      = F(0) + (nums[0]+nums[1]..+nums[n-2]) - nums[n-1] * n-1
//      = F(0) + (nums[0]+nums[1]..+nums[n-2]+nums[n-1]) - nums[n-1] * n
//      = F(0) + total_sum - nums[n-1] * n
// F(k) = F(k-1) + total_sum - nums[n-k] * n
impl Solution {
    pub fn max_rotate_function(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        let mut total_sum = 0;
        let mut F = 0;
        for i in 0..n{
            F += (i as i32) * nums[i];
            total_sum += nums[i];
        }
        // println!("{}:{}", 0, F);
        let mut ans = F;
        for i in 1..n {
            F = F + total_sum - nums[n-i] * n as i32;
            // println!("{}:{}", i, F);
            ans = ans.max(F);
        }
        ans
    }
}
