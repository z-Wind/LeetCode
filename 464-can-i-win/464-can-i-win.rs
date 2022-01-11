// https://leetcode.com/problems/can-i-win/discuss/95277/Java-solution-using-HashMap-with-detailed-explanation

use std::collections::HashMap;
impl Solution {
    pub fn can_i_win(max_choosable_integer: i32, desired_total: i32) -> bool {
        if desired_total == 0 {
            return true;
        }
        if (max_choosable_integer+1)*max_choosable_integer/2 < desired_total{
            return false;
        }
        
        let mut dp = HashMap::new();
        check(&mut dp, max_choosable_integer, 0, desired_total)
    }
}

fn check(dp:&mut HashMap<i32, bool>, n: i32, state: i32, desired_total: i32) -> bool{    
    if dp.contains_key(&state){
        return *dp.get(&state).unwrap();
    }
    
    let mut ans = false;
    // println!("{}: {}, {:?}", player, desired_total, nums);
    for i in 0..n {
        let shift = (1<<i);
        if (state & shift) != 0 {
            continue;  
        } 
        
        let desired_total = desired_total - (i+1) as i32;
        if desired_total <= 0 || !check(dp, n, state | shift, desired_total){
            ans = true;
            break;
        }
    }
    
    dp.insert(state, ans);
    
    ans
}