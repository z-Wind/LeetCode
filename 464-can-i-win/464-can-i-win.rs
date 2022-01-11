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
        
        let mut nums:Vec<bool> = vec![true;max_choosable_integer as usize];
        let mut dp = HashMap::new();
        check(&mut dp, &mut nums, desired_total, 1)
    }
}

fn check(dp:&mut HashMap<i32, bool>, nums: &mut Vec<bool>, desired_total: i32, player: i32) -> bool{    
    let s = format(nums);
    if dp.contains_key(&s){
        return *dp.get(&s).unwrap();
    }
    
    let mut ans = false;
    let n = nums.len();
    // println!("{}: {}, {:?}", player, desired_total, nums);
    for i in (0..n).rev() {
        if !nums[i]{
            continue;
        }
        nums[i] = false;
        let desired_total = desired_total - (i+1) as i32;
        if desired_total <= 0 || !check(dp, nums, desired_total, player%2 + 1){
            ans = true;
            nums[i] = true;
            break;
        }
        nums[i] = true;
    }
    
    dp.insert(s, ans);
    
    ans
}

// transfer boolean[] to an Integer 
fn format(used: &Vec<bool>) -> i32{
    let mut num = 0;
    for &b in used {
        num <<= 1;
        if b {
            num |= 1;
        }
    }
    return num;
}