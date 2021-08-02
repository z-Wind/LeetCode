impl Solution {
    pub fn int_to_roman(mut num: i32) -> String {
        let mut s = String::from("");
        while num > 0{
            match num{
                x if x>=1000 => {
                    s.push_str(&"M".repeat(num as usize/1000));
                    num %= 1000
                }
                x if x>=900 => {
                    num-=900;
                    s.push_str("CM");
                },
                x if x>=500 => {
                    num-=500;
                    s.push('D');
                },
                x if x>=400 => {
                    num-=400;
                    s.push_str("CD");
                },
                x if x>=100 => {
                    s.push_str(&"C".repeat(num as usize/100));
                    num %= 100
                },
                x if x>=90 => {
                    num-=90;
                    s.push_str("XC");
                },
                x if x>=50 => {
                    num-=50;
                    s.push('L');
                },
                x if x>=40 => {
                    num-=40;
                    s.push_str("XL");
                },
                x if x>=10 => {
                    s.push_str(&"X".repeat(num as usize/10));
                    num %= 10
                },
                x if x>=9 => {
                    num-=9;
                    s.push_str("IX");
                },
                x if x>=5 => {
                    num-=5;
                    s.push('V');
                },
                x if x>=4 => {
                    num-=4;
                    s.push_str("IV");
                },
                x if x>=1 => {
                    s.push_str(&"I".repeat(num as usize));
                    num = 0;
                },
                _ => panic!(),
            }
        }
        s
    }
}