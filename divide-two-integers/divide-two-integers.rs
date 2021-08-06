impl Solution {
    pub fn divide(dividend: i32, divisor: i32) -> i32 {
        match (dividend, divisor){
            (0,_) => 0,
            (i32::MIN,i32::MIN) => 1,
            (i32::MAX,i32::MAX) => 1,
            (i32::MIN,i32::MAX) => -1,
            (_,i32::MIN) => 0,          
            (_,i32::MAX) => 0,
            (q,1) => q,
            (i32::MIN,-1) => i32::MAX,
            (q @ i32::MIN..=i32::MAX,-1) => -q,
            (num, div) => quotient(num, div),
        }
    }
}

fn quotient(num:i32, div:i32) -> i32{
    let sign = if (1 == (1 & num >> 31) ^ (1 & div >> 31)){-1} else {1};
    let mut num = if num > 0 {-num} else {num};
    let mut div = if div > 0 {-div} else {div};
    //if div > num {return 0};
    
    //println!("num:{}, div:{}",num,div);
    let mut q = 0;
    while num <= div{
        let mut shift = 0;
        let mut val = div;
        let mut minus = false;
        while num <= val{
            if val < (i32::MIN>>1){
                minus = false;
                break;
            }
            shift += 1;
            val<<=1;
            minus = true;
        }
        if minus{
            shift-=1;
            val>>=1;
        }
        //println!("before: num:{}, val:{}, shift:{}",num,val,shift);
        num -= val;
        //println!("after: num:{}, val:{}, shift:{}",num,val,shift);
        q += 1<<shift;
    }
    
    q*sign
}