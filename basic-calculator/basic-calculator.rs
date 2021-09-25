impl Solution {
    pub fn calculate(s: String) -> i32 {
        calculate(s.as_bytes())
    }
}

fn calculate(s: &[u8]) -> i32 {
    // println!("{}", String::from_utf8(s.to_vec()).unwrap());
    let mut sign = 1;
    let mut lhs = String::new();
    let mut paren = 0;
    let mut i = 0;
    while i < s.len(){
        match s[i]{
            b'+' if paren == 0 => {
                return calculate(lhs.as_bytes()) * sign + calculate(&s[i+1..]);
            },
            b'-' if paren == 0 => {
                if lhs.is_empty(){
                    sign = -1;
                } else {
                    return calculate(lhs.as_bytes()) * sign + calculate(&s[i..]);
                }
            },
            b'(' => {
                if paren != 0{
                    lhs.push('(')
                }
                paren += 1;
            },
            b')' => {
                paren -= 1;
                if paren != 0{
                    lhs.push(')')
                }
            }
            b' ' => (),
            c => lhs.push(c as char),
        }
        i+=1;
    }
    match lhs.parse::<i32>(){
        Ok(num) => num * sign,
        Err(_) => calculate(lhs.as_bytes()) * sign,
    }
}
