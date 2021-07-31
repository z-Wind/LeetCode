impl Solution {
    pub fn reverse(x: i32) -> i32 {
        if -10 < x && x < 10{
            return x;
        }
        
        let mut temp = x;
        let mut ans = 0;
        while temp != 0{
            let pop = temp%10;
            temp /= 10;
            
            if (x >=0 && ans > (i32::MAX - pop)/10) || 
               (x < 0 && ans < (i32::MIN - pop)/10) {
                return 0;
            }
            
            ans = ans * 10 + pop;
        }
        
        ans
    }
}