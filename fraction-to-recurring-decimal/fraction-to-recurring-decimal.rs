impl Solution {
    pub fn fraction_to_decimal(numerator: i32, denominator: i32) -> String {
        if numerator == 0{
            return String::from("0");
        }
        let sign = match (numerator<0,denominator<0){
            (false,false) | (true,true) => 1,
            _ => -1,
        };
        
        let mut numerator = (numerator as i64).abs();
        let mut denominator = (denominator as i64).abs();
        
        let i = numerator/denominator;
        numerator %= denominator;
        if numerator == 0{
            return add_sign(sign, i.to_string());
        }
        
        let mut s = vec![];
        while numerator != 0{
            numerator *= 10;
            s.push((numerator,denominator));
            match check_repeat(&s[..]){
                None => (),
                Some((start,end)) => {
                    let f = s.iter().map(|(num,den)| (num/den).to_string()).collect::<Vec<String>>().join("");
                    let r = &f[start..=end];
                    return add_sign(sign,format!("{}.{}({})",i,f.replace(r,""),r));
                },
            }
            
            numerator %= denominator;
        }
        
        let f = s.iter().map(|(num,den)| (num/den).to_string()).collect::<Vec<String>>().join("");
        add_sign(sign, i.to_string() + "." + &f)
    }
}

fn check_repeat(s:&[(i64,i64)]) -> Option<(usize,usize)>{
    let len = s.len();
    for i in (1..=len/2){        
        // println!("{:?} ==? {:?}", &s[len-i..], &s[len-i*2..len-i]);
        if s[len-i..] == s[len-i*2..len-i]{
            return Some((len-i,len-1));
        }
    }
    return None;
}

fn add_sign(sign:i32, s:String) -> String{
    if sign == 1{
        return s
    }
    "-".to_owned() + &s
}