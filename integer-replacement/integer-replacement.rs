// https://leetcode.com/problems/integer-replacement/discuss/87920/A-couple-of-Java-solutions-with-explanations
// 111011 -> 111100 -> 11110 -> 1111 -> 10000 -> 1000 -> 100 -> 10 -> 1
// the rightmost should have more zeros
// like this
// 1001 -> 1000        -> 100 -> 10 -> 1
// 1001 -> 1010 -> 101 -> 100 -> 10 -> 1

impl Solution {
    pub fn integer_replacement(n: i32) -> i32 {
        let mut n = n as i64;
        let mut count = 0;
        while n > 1{
            // println!("{:b}", n);
            if n & 1 == 0{ // even is fastest
                n >>= 1;
            // make num to 0b*00
            } else if n == 3 || (n>>1) & 1 == 0{ // 0b11 or 0b*01
                n-=1;
            } else {
                n+=1;
            }
            count += 1;
        }
        count
    }
}
