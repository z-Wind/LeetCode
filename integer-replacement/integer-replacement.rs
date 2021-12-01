// 1 -> 2 -> 4 -> 8 -> 16 -> 32 -> 64 -> 128
use std::collections::HashMap;
impl Solution {
    pub fn integer_replacement(n: i32) -> i32 {
        let mut dp: HashMap<i64, i32> = HashMap::new();
        integer_replacement(&mut dp, n as i64)
    }
}

fn integer_replacement(dp: &mut HashMap<i64, i32>, n: i64) -> i32 {
    // println!("{}", n);
    if n == 1 {
        return 0;
    }
    if dp.contains_key(&n) {
        return *dp.get(&n).unwrap();
    }

    let count = {
        if n & 1 == 0 {
            // n%2 == 0
            integer_replacement(dp, n >> 1)
        } else {
            integer_replacement(dp, n - 1).min(integer_replacement(dp, n + 1))
        }
    } + 1;
    dp.insert(n, count);
    count
}
