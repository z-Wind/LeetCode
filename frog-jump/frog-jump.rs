// https://leetcode.com/problems/frog-jump/discuss/193816/Concise-and-fast-DP-solution-using-2D-array-instead-of-HashMap-with-text-and-video-explanation.

// let dp(i) denote a set containing all next jump size at stone i
// The maximum jump size the frog can make at each stone if possible is shown as followings: 
// stone:      0, 1, 2, 3, 4, 5
// jump size:  1, 2, 3, 4, 5, 6 (suppose frog made jump with size k + 1 at each stone)

impl Solution {
    pub fn can_cross(stones: Vec<i32>) -> bool {
        let N = stones.len();
        let mut dp:Vec<Vec<bool>> = vec![vec![false;N+1];N];
        dp[0][1] = true;
        
        for i in 1..N{
            for j in 0..i{
                let diff = (stones[i] - stones[j]) as usize;
                if diff > N || !dp[j][diff]{
                    continue;
                }
                dp[i][diff] = true;
                if diff >= 1{
                    dp[i][diff - 1] = true;
                }
                if diff + 1 <= N{
                    dp[i][diff + 1] = true;
                }
                if i == N - 1 {
                    return true;
                }
            }
        }

        return false;
    }
}