//   1~9   =>   9*1
//  10~99  =>  90*2
// 100~999 => 900*3

impl Solution {
    pub fn find_nth_digit(n: i32) -> i32 {
        find_nth_digit(n as i64, 1)
    }
}

fn find_nth_digit(n: i64, level: i64) -> i32 {
    let start = 10_i64.pow(level as u32 - 1);
    let all = 9 * level * start;
    if n > all {
        return find_nth_digit(n - all, level + 1);
    }
    
    let idx = (n - 1) / level;
    let loc = ((n - 1) % level) as usize;
    let num = start+idx;
    (num.to_string().as_bytes()[loc] - b'0') as i32
}
