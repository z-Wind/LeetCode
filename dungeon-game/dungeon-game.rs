// https://leetcode.com/problems/dungeon-game/discuss/745340/post-Dedicated-to-beginners-of-DP-or-have-no-clue-how-to-start

// dp[i,j] = min(dp[i+1][j], dp[i][j+1]) - dungeon[i][j]
// 
// dp[i,j]   dp[i][j+1]
// dp[i+1,j]
//
// dp[j](update) dp[j+1]
// dp[j]
//

impl Solution {
    pub fn calculate_minimum_hp(dungeon: Vec<Vec<i32>>) -> i32 {
        let m = dungeon.len();
        let n = dungeon[0].len();
        let mut dp = vec![0;n];
        
        dp[n-1] = health(-dungeon[m-1][n-1] + 1);
        for j in (0..n-1).rev(){
            dp[j] = health(dp[j+1]-dungeon[m-1][j]);
        }
        
        for i in (0..m-1).rev(){
            dp[n-1] = health(dp[n-1]-dungeon[i][n-1]);
            for j in (0..n-1).rev(){
                dp[j] = health(dp[j].min(dp[j+1]) - dungeon[i][j]);
            }
        }

        dp[0]
    }
}

fn health(hp:i32) -> i32{
    if hp <= 0 {1}
    else { hp }
}
