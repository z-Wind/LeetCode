impl Solution {
    pub fn climb_stairs(n: i32) -> i32 {
        if n == 1{
            return 1;
        }
        let n  = n as usize;
        // dp(n) = dp(n-1) + dp(n-2)
        let mut step:Vec<i32> = vec![1;2];
        for i in (2..n+1){
            //println!("{:?}", step);
            let sum = step[0] + step[1];
            step[0] = step[1];
            step[1] = sum;
        }
        step[1]
    }
}