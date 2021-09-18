use std::collections::HashSet;
impl Solution {
    pub fn is_happy(mut n: i32) -> bool {
        let mut set = HashSet::new();
        set.insert(n);
        while n != 1{
            let mut sum = 0;
            while n != 0{
                sum += (n%10).pow(2);
                n/=10;
            }
            n = sum;
            if set.contains(&n){
                return false;
            }
            set.insert(n);
        }   
        true
    }
}