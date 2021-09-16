// dp[i][j] = max(dungeon[i][j] + dp[i-1][j], dungeon[i][j] + dp[i][j-1])
//
//               dp[i-1][j]
//  dp[i][j-1]   dp[i][j]
//
//               dp[j]
//  dp[j-1]      dp[j](update)
//
impl Solution {
    pub fn calculate_minimum_hp(dungeon: Vec<Vec<i32>>) -> i32 {
        let m = dungeon.len();
        let n = dungeon[0].len();
        let mut dp = vec![vec![(i32::MAX,0_i32)];n];
        
        dp[0] = update(&dp[0], dungeon[0][0]);
        for j in (1..n){
            dp[j] = update(&dp[j-1], dungeon[0][j]);
        }
        
        for i in (1..m){
            // println!("{:?}", dp);
            dp[0] = update(&dp[0], dungeon[i][0]);
            for j in (1..n){
                let from_up = update(&dp[j], dungeon[i][j]);
                let from_left = update(&dp[j-1], dungeon[i][j]);
                dp[j] = compare(from_left, from_up);
            }
        }
        
        // println!("{:?}", dp);
        let ans = dp[n-1].iter().max_by_key(|x| x.0).unwrap();
        if ans.0 >= 0{
            return 1;
        } else {
            return -ans.0+1;
        }
    }
}

fn compare(mut a:Vec<(i32,i32)>, mut b:Vec<(i32,i32)>) -> Vec<(i32,i32)>{
    let mut result = a;
    for v in b{
        let mut push = false;
        for i in (0..result.len()).rev(){
            if v.0 >= result[i].0 && v.1 >= result[i].1{
                result.remove(i);
                push = true;
            } else if v.0 < result[i].0 && v.1 > result[i].1 ||
                      v.0 > result[i].0 && v.1 < result[i].1{
                push = true;
            } else {//if v.0 <= result[i].0 && v.1 <= result[i].1 {
                push = false;
                break;
            }
        }
        if push{
            result.push(v);
        }
    }
    // println!("{:?}", result);
    result
}

fn update(dp:&Vec<(i32,i32)>, val:i32) -> Vec<(i32,i32)>{
    let mut v = vec![];
    for e in dp{
        let hp = e.1+val;
        v.push((e.0.min(hp), hp));
    }
    v
}
