// https://leetcode.com/problems/dungeon-game/discuss/745340/post-Dedicated-to-beginners-of-DP-or-have-no-clue-how-to-start

// dp[i,j] = min(dp[i+1][j], dp[i][j+1]) - dungeon[i][j]
// 
// dp[i,j]   dp[i][j+1]
// dp[i+1,j]
//

impl Solution {
    pub fn calculate_minimum_hp(dungeon: Vec<Vec<i32>>) -> i32 {
        let m = dungeon.len();
        let n = dungeon[0].len();
        let mut dp = vec![vec![0;n];m];
        
        dp[m-1][n-1] = health(-dungeon[m-1][n-1] + 1);
        
        for i in (0..m-1).rev(){
            dp[i][n-1] = health(dp[i+1][n-1]-dungeon[i][n-1]);
        }
        for j in (0..n-1).rev(){
            dp[m-1][j] = health(dp[m-1][j+1]-dungeon[m-1][j]);
        }
        
        for i in (0..m-1).rev(){
            for j in (0..n-1).rev(){
                dp[i][j] = health(dp[i+1][j].min(dp[i][j+1]) - dungeon[i][j]);
            }
        }
        // println!("{:?}", dp);
        dp[0][0]
    }
}

fn health(hp:i32) -> i32{
    if hp <= 0 {1}
    else { hp }
}
