//16 8 4 => repeat
// 0 0 0
// 0 0 1
// 0 1 1
// 0 1 0
// 1 
// 1
// 1
// 1
impl Solution {
    pub fn gray_code(n: i32) -> Vec<i32> {
        let mut ans:Vec<i32> = vec![];
        for i in (0..2i32.pow(n as u32)){
            let mut num = 0;
            for j in (0..n){
                num |= (get_bit(i>>j) << j)
            }
            //println!("{} => {:0n$b}", i, num, n=n as usize);
            ans.push(num);
        }
        ans
    }
}

fn get_bit(n: i32) -> i32{
    match n%4{
        0 | 3 => 0,
        1 | 2 => 1,
        _ => panic!(),
    }
}