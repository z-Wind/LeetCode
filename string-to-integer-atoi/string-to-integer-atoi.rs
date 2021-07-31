impl Solution {
    pub fn my_atoi(s: String) -> i32 {
        let mut temp:Vec<char> = vec!{};
        let mut flag = true;
        for c in s.chars(){
            match c{
                ' ' if flag => continue,
                '+' | '-' if flag => {
                    temp.push(c);
                    flag = false;
                }
                '0'..='9' => {
                    temp.push(c);
                    flag = false;
                },
                _ => break,
            }
        }
        
        let s:String = temp.into_iter().collect();
        //println!("{}", s);

        match s.parse(){
            Ok(x) => x,
            Err(e) if e.to_string().contains("too small") => i32::MIN,
            Err(e) if e.to_string().contains("too large") => i32::MAX,
            Err(e) => {
                //println!("{}", e);
                0
            },
                
        }
    }
}