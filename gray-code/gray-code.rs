// 2^b * x + 2^(b-1) = i
impl Solution {
    pub fn gray_code(n: i32) -> Vec<i32> {
        let mut ans:Vec<i32> = vec![0];
        let mut num = ans[0];
        for i in (1..2i32.pow(n as u32)){
            num = change_bit(&num, i, n);
            //println!("{} => {:0n$b}", i, num, n=n as usize);
            ans.push(num);
        }
        ans
    }
}

fn change_bit(num: &i32, i:i32, n:i32) -> i32{
    let mut bit_n:i32 = 1;
    // 2^b * x + 2^(b-1) = i
    for b in (1..=n){
        if (i-(1<<(b-1))) % (1<<b) == 0{
            //println!("{} => b:{}",i, b);
            bit_n = (1 << (b-1));
            break;
        }
    }
    //println!("{:0n$b} ^ {:0n$b} = {:0n$b}", num, bit_n, *num ^ bit_n,n=n as usize);
    *num ^ bit_n
}