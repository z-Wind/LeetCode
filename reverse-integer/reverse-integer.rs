impl Solution {
    pub fn reverse(x: i32) -> i32 {
        if -10 < x && x < 10{
            return x;
        }
        
        let mul = if x < 0 {-1} else {1};
        
        let s: String = (x*mul).to_string();        
        let t: String = s.chars().rev().collect();
        
        //println!("{} => {}",s,t);
        
        let ans = match t.parse(){
            Ok(x) => x,
            Err(_) => 0,
        };
        
        return ans* mul;
    }
}