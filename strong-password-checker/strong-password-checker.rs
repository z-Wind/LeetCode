impl Solution {
    pub fn strong_password_checker(password: String) -> i32 {
        let mut n = password.len() as i32;
        let mut lower = 0;
        let mut upper = 0;
        let mut digit = 0;
        let chars = password.as_bytes();
        for c in chars.iter(){
            if c.is_ascii_lowercase(){
                lower += 1
            } else if c.is_ascii_uppercase(){
                upper += 1
            } else if c.is_ascii_digit(){
                digit += 1
            }  
        }
        checker(chars, 1, n, 0, 1, lower, upper, digit)
    }
}

fn checker(pw: &[u8], start:usize, mut n:i32, step:i32, mut count:i32, lower:i32, upper:i32, digit:i32) -> i32{
    if start >= pw.len(){
        // println!("len:{}, step:{}, lower:{}, upper:{}, digit:{}, count:{}",n,step,lower,upper,digit,count);
        let mut plus = 0;
        if lower < 1{
            plus+=1;
        }
        if upper < 1{
            plus+=1;
        }
        if digit < 1{
            plus+=1;
        }
        if n < 6{
            n+=plus;
        }
        return step + 0.max(6-n) + 0.max(n-20) + plus;
    }
    
    if pw[start] == pw[start-1]{
        count += 1;
    } else {
        count = 1;
    }
    // println!("{}:{} => len:{}, step:{}, lower:{}, upper:{}, digit:{}, count:{}",start,pw[start] as char,n,step,lower,upper,digit,count);
    if count < 3{
        return checker(pw, start+1, n, step, count, lower, upper, digit);
    }
    let mut ans = i32::MAX;
    
    // insert
    if n<6 || lower < 1{
        ans = ans.min(checker(pw, start+2, n+1, step+1, 1, lower+1, upper, digit));
    } else if upper < 1{
        ans = ans.min(checker(pw, start+2, n+1, step+1, 1, lower, upper+1, digit));
    } else if digit < 1{
        ans = ans.min(checker(pw, start+2, n+1, step+1, 1, lower, upper, digit+1));
    }
    
    // delete
    if n>20 || start == pw.len()-1 || (start+1 < pw.len() && pw[start] != pw[start+1]){
        if pw[start].is_ascii_lowercase(){
            ans = ans.min(checker(pw, start+1, n-1, step+1, count-1, lower-1, upper, digit));
        } else if pw[start].is_ascii_uppercase(){
            ans = ans.min(checker(pw, start+1, n-1, step+1, count-1, lower, upper-1, digit));
        } else if pw[start].is_ascii_digit(){
            ans = ans.min(checker(pw, start+1, n-1, step+1, count-1, lower, upper, digit-1));
        } else {
            ans = ans.min(checker(pw, start+1, n-1, step+1, count-1, lower, upper, digit));
        }
    }
    
    // replace
    if pw[start].is_ascii_lowercase(){
        if upper < 1{
            ans = ans.min(checker(pw, start+2, n, step+1, 1, lower-1, upper+1, digit));
        } else if digit < 1{
            ans = ans.min(checker(pw, start+2, n, step+1, 1, lower-1, upper, digit+1));
        } else {
            ans = ans.min(checker(pw, start+2, n, step+1, 1, lower, upper, digit));
        }
    } else if pw[start].is_ascii_uppercase(){
        if lower < 1{
            ans = ans.min(checker(pw, start+2, n, step+1, 1, lower+1, upper-1, digit));
        } else if digit < 1{
            ans = ans.min(checker(pw, start+2, n, step+1, 1, lower, upper-1, digit+1));
        } else {
            ans = ans.min(checker(pw, start+2, n, step+1, 1, lower, upper, digit));
        }
    } else if pw[start].is_ascii_digit(){
        if upper < 1{
            ans = ans.min(checker(pw, start+2, n, step+1, 1, lower, upper+1, digit-1));
        } else if lower < 1{
            ans = ans.min(checker(pw, start+2, n, step+1, 1, lower+1, upper, digit-1));
        } else {
            ans = ans.min(checker(pw, start+2, n, step+1, 1, lower, upper, digit));
        }
    } else {
        if upper < 1{
            ans = ans.min(checker(pw, start+2, n, step+1, 1, lower, upper+1, digit));
        } else if lower < 1{
            ans = ans.min(checker(pw, start+2, n, step+1, 1, lower+1, upper, digit));
        } else if digit < 1 {
            ans = ans.min(checker(pw, start+2, n, step+1, 1, lower, upper, digit+1));
        } else {
            ans = ans.min(checker(pw, start+2, n, step+1, 1, lower, upper, digit));
        }
    }
    
    ans
}