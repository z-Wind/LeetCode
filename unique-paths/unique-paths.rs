use std::collections::HashMap;

impl Solution {
    pub fn unique_paths(m: i32, n: i32) -> i32 {
        let mut uni = UNI::new();
        uni.unique_paths(m,n)
    }
}

struct UNI{
    dp:HashMap<(i32,i32), i32>,
}

impl UNI{
    fn new() -> Self{
        UNI{
            dp: HashMap::new(),
        }
    }
    fn unique_paths(&mut self, m: i32, n: i32) -> i32 {
        if m == 1 && n == 1{
            return 1;
        } else if m == 0 || n == 0{
            return 0;
        }
        
        if self.dp.contains_key(&(m,n)){
            return *self.dp.get(&(m,n)).unwrap();
        }
        
        let step = self.unique_paths(m-1,n) + self.unique_paths(m,n-1);
        self.dp.insert((m,n), step);
        self.dp.insert((n,m), step);
        
        step
    }
}