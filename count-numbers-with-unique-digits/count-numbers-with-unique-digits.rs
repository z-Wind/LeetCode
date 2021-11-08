// x    => a:10
// xx   => ab:10*9
//         0b:c(1,1) => means 00
// xxx  => abc:10*9*8
//         0bc:c(2,1)*9 => means 00x or 0x0
//         00c:c(1,1) => means 000
// xxxx => abcd:10*9*8*7
//         0bcd:c(3,1)*9*8 => means 00xx or 0x0x or 0xx0
//         00cd:c(2,1)*9 => means 000x, or 00x0
//         000d:c(1,1) => means 0000
impl Solution {
    pub fn count_numbers_with_unique_digits(n: i32) -> i32 {
        let mut count = 0;
        
        // all diff: abcd..
        let mut ans = 1;
        for i in 0..n{
            ans *= (10-i);
        }
        count += ans;
        // println!("10*9*8..= {}", ans);
        
        for i in (1..=n-1).rev(){
            ans = c(i,1);
            for num in 10-(i-1)..=9{
                ans *= num;    
            }
            // println!("c({},1) * {:?} = {}", i, 9-(n-1-i)+1..=9, ans);
            count += ans;
        }
        count
    }
}

fn c(n:i32,m:i32) -> i32{
    let mut ans = 1;
    for i in n-m+1..=n{
        ans *= i;
    }
    for i in 2..=m{
        ans /= i;
    }
    ans
}