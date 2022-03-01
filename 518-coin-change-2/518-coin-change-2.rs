// https://github.com/z-Wind/LeetCode/blob/main/combination-sum/combination-sum.rs

use std::collections::HashMap;
impl Solution {
    pub fn change(amount: i32, mut coins: Vec<i32>) -> i32 {
        if amount == 0 {
            return 1;
        }
        coins.sort_unstable();
        let mut map = HashMap::new();
        explore(&mut map, &coins, amount, 0, 0)
    }
}

fn explore(
    map: &mut HashMap<(usize, i32), i32>,
    coins: &Vec<i32>,
    target: i32,
    i_start: usize,
    cur_sum: i32,
) -> i32 {
    let key = (i_start, cur_sum);
    if let Some(x) = map.get(&key) {
        return *x;
    }
    // println!("{} {}", coins[i_start], cur_sum);
    let mut ans = 0;
    for i in i_start..coins.len() {
        let sum = cur_sum + coins[i];
        if sum == target {
            ans += 1;
        } else if sum < target {
            ans += explore(map, coins, target, i, sum);
        } else {
            break;
        }
    }
    map.insert(key, ans);
    ans
}
