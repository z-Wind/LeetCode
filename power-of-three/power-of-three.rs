impl Solution {
    pub fn is_power_of_three(n: i32) -> bool {
        if n >= 1162261467{
            return n == 1162261467;
        }
        
        let mut m = 1;
        while m < n{
            m = m+m+m;
            // println!("{}",m);
        }
        
        m == n
    }
}