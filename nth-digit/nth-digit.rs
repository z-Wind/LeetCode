//   1~9   =>   9*1
//  10~99  =>  90*2
// 100~999 => 900*3

impl Solution {
    pub fn find_nth_digit(n: i32) -> i32 {
        find_nth_digit(n as i64, 1) as i32
    }
}

fn find_nth_digit(n: i64, level: i64) -> i64 {
    let start = 10_i64.pow(level as u32 - 1);
    let all = 9 * level * start;
    if n > all {
        return find_nth_digit(n - all, level + 1);
    }
    
    let idx = ((n - 1) / level) as usize;
    let mut loc = level - 1 - (n - 1) % level;
    let mut num = (start..).nth(idx).unwrap();
    // println!("n:{} ({}..) => idx:{}, num:{}, loc:{}",n,start, idx, num, loc);
    while loc > 0 {
        num /= 10;
        loc -= 1;
    }
    num % 10
}
