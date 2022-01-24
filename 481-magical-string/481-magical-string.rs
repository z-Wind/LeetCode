impl Solution {
    pub fn magical_string(n: i32) -> i32 {
        if n == 1 || n == 2{
            return 1;
        }
        
        let n = n as usize;
        let mut s = Vec::with_capacity(n);
        s.push(1);
        s.push(2);
        
        let mut cur = 0;
        let mut count_i = 0;
        let mut ele = 1;
        let mut ans = 0;
        while s.len() < n { 
            if ele == 1{
                ans += s[count_i];
            }
            
            let mut count = 0;
            while cur < s.len() && s[cur] == ele {
                count += 1;
                cur += 1;
            }
            for _ in count..s[count_i]{
                s.push(ele);
                cur += 1;
            }
            count_i += 1;
            ele ^= 0b11;
        }
        if ele == 2{
            ans -= (s.len() - n);
        }
        // println!("{:?}", s);        
        
        ans as i32
    }
}
