// (1) d[i] = d[i-2] + nums[i] if robbing, 
// (2) d[i] = d[i-1]           if not robbing
// d[i] = max(dp[i-1], d[i-2] + nums[i])
// 
// d[i-2] d[i-1] d[i]
//
// pre    cur    cur(update)
//
impl Solution {
    pub fn rob(nums: Vec<i32>) -> i32 {
        if nums.len() == 1{
            return nums[0];
        }
        let mut ans = rob(&nums[1..]);
        ans.max(rob(&nums[..nums.len()-1]))
    }
}

fn rob(nums: &[i32]) -> i32 {
    let n = nums.len();

    let mut pre = 0;
    let mut cur = 0;
    for i in (0..n){
        let temp = cur;
        cur = cur.max(pre+nums[i]);
        pre = temp;
    }

    cur
}