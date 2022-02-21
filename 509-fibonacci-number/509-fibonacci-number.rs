impl Solution {
    pub fn fib(n: i32) -> i32 {
        if n == 0 || n == 1 {
            return n;
        }
        
        let mut ele = (0, 1);
        for _ in 2..=n {
            ele = (ele.1, ele.0 + ele.1);
        } 
        
        ele.1
    }
}
