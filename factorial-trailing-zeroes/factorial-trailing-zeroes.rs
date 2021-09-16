impl Solution {
    pub fn trailing_zeroes(n: i32) -> i32 {
        let mut count = 0;
        for mut num in (1..=n){
            while num%5 == 0{
                count+=1;
                num/=5;
            }
        }
        count
    }
}