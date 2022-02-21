use std::collections::HashMap;

impl Solution {
    pub fn fib(n: i32) -> i32 {
        let mut mem = HashMap::new();
        fib(n, &mut mem)
    }
}

fn fib(n: i32, mem: &mut HashMap<i32, i32>) -> i32 {
    if n == 0 || n == 1 {
        return n;
    }

    if mem.contains_key(&n) {
        return *mem.get(&n).unwrap();
    }

    fib(n - 1, mem) + fib(n - 2, mem)
}
