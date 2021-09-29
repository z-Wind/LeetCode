// 10
// 1x => 1
// x1 => 1
// 2
//
// 13
// 1x => 4
// x1 => 2
// 6
//
// 100
// 1xx => 1
// x1x => 1*10 = 10
// xx1 => 1*10 = 10
// 21
//
// 159
// 1xx => 60
// x1x => 2*10 = 20
// xx1 => 1*6+1*10 = 16
// 96
//
// 2248
// 1xxx => 10*10*10 = 1000
// x1xx => 3*10*10 = 300
// xx1x => 1*3*10 + 2*10*10 = 230
// xxx1 => 1*(1*5+2*10) + 2*10*10 = 225
// 1755
//
// 42015
// 1xxxx => 10*10*10*10 = 10000
// x1xxx => 5*10*10*10 = 5000
// xx1xx => 42*10*10 = 4200
// xxx1x => 420*10 + 1 * 6 = 4206
// xxxx1 => 4202
// 27608
// 
// if cur_digit bigger than 1 => prefix all+1, suffix all 10 
// if cur_digit less than 1 => prefix all, suffix all 10
// if cur_digit equal 1 => two part 
        // 1. prefix all, suffix all 10 
        // 2. prefix 1, suffix all+1
impl Solution {
    pub fn count_digit_one(n: i32) -> i32 {    
        let mut digits = 0;
        let mut count = 0;
        let mut prefix = n;
        let mut suffix = 0;
        while prefix != 0{
            let cur_digit = prefix%10;
            prefix/=10;
            match (cur_digit, prefix, suffix){
                (2..=9, prefix, suffix) => {
                    count += (prefix+1) * 10_i32.pow(digits);
                },
                (0, prefix, suffix) => {
                    count += prefix * 10_i32.pow(digits);
                },
                (1, prefix, suffix) => {
                    count += prefix * 10_i32.pow(digits);
                    count += 1 * (suffix+1);
                },
                x => panic!("{:?}",x),
            }
            // println!("{:?} => {} {}",  (cur_digit, prefix, suffix),count,digits);
            suffix += cur_digit*10_i32.pow(digits);
            digits += 1;
        }
        
        count
    }
}