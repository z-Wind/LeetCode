impl Solution {
    pub fn decode_string(s: String) -> String {
        let mut ans = String::new();
        let mut count:usize = 0;
        let mut stack = Vec::new();
        
        for c in s.chars(){
            match c{
                'a'..='z' => ans.push(c),
                '0'..='9' => count = count * 10 + ((c as u8) - b'0') as usize,
                '[' => {
                    stack.push((ans,count));
                    count = 0;
                    ans = String::new();
                },
                ']' => {
                    let (mut s, r) = stack.pop().unwrap();
                    s.push_str(&ans.repeat(r));
                    ans = s;
                },
                _ => panic!("impossible"),
            }
            // println!("{} => ans:{},count:{},stack:{:?}",c,ans,count,stack);
        }
        
        ans
    }
}