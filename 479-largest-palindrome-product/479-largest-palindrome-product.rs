// https://leetcode.com/problems/largest-palindrome-product/discuss/96306/Java-solutions-with-two-different-approaches

impl Solution {
    pub fn largest_palindrome(n: i32) -> i32 {
        if n == 1{
            return 9;
        }
        
        let n = n as u32;
        let start = 10_i64.pow(n-1);
        let end = 10_i64.pow(n);
        for prefix in (start..end).rev() {
            let num = prefix * end + reverse(prefix);
            // println!("{}", num);
            for x in (start..end).rev() {
                if num / x > x {
                    break;
                } else if num % x == 0 {
                    println!("{}", num);
                    return (num % 1337) as i32;
                }
            }
        }
        
        panic!("no answer");
    }
}

// https://github.com/z-Wind/LeetCode/blob/main/reverse-integer/reverse-integer.rs
fn reverse(x: i64) -> i64 {
    if -10 < x && x < 10{
        return x;
    }

    let mut temp = x;
    let mut ans = 0;
    while temp != 0{
        let pop = temp % 10;
        temp /= 10;

        if (x >=0 && ans > (i64::MAX - pop)/10) || 
           (x < 0 && ans < (i64::MIN - pop)/10) {
            return 0;
        }

        ans = ans * 10 + pop;
    }

    ans
}