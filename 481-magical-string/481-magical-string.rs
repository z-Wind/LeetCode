impl Solution {
    pub fn magical_string(n: i32) -> i32 {
        if n == 1 || n == 2{
            return 1;
        }
        
        let n = n as usize;
        let mut s = vec![0;n+1];
        s[0] = 1;
        s[1] = 2;
        
        let mut cur = 0;
        let mut count_i = 0;
        let mut ele = 1;
        let mut ans = 0;
        while cur < n { 
            for _ in 0..s[count_i]{
                s[cur] = ele;
                if ele == 1 && cur < n {
                    ans += 1;   
                }
                cur += 1;
            }
            count_i += 1;
            ele ^= 0b11;
        }
        // println!("{:?}", s);        
        
        ans as i32
    }
}
