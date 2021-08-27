impl Solution {
    pub fn restore_ip_addresses(s: String) -> Vec<String> {
        let mut cur:Vec<u8> = Vec::new();
        let mut ans:Vec<Vec<u8>> = vec![];
        backtrack(&mut ans, s.as_bytes(), &mut cur, 0, 0);
        //println!("{:?}", ans);
        ans.iter().map(|b| String::from_utf8(b[..].to_vec()).unwrap()).collect()
    }
}


fn backtrack(ans:&mut Vec<Vec<u8>>, s:&[u8], cur:&mut Vec<u8>, pos:usize, dot_num:i32){
    //println!("{}: {}", pos, String::from_utf8(cur.to_vec()).unwrap());
    if pos == s.len() && dot_num==4{
        //println!("clone");
        ans.push(cur[..cur.len()-1].to_vec());
        return;
    } else if  pos == s.len() || dot_num == 4{
        return;
    }
    
    // 0~9
    match s[pos]{
        b'0'..=b'9' => {
            cur.push(s[pos]);
            cur.push(b'.');
            backtrack(ans,s,cur,pos+1,dot_num+1);
            cur.pop();
            cur.pop();
        },
        _ => (),
    }
    if pos == s.len() - 1{
        return;
    }
    // 10~99
    match (s[pos],s[pos+1]){
        (b'1'..=b'9', b'0'..=b'9') => {
            cur.push(s[pos]);
            cur.push(s[pos+1]);
            cur.push(b'.');
            backtrack(ans,s,cur,pos+2,dot_num+1);
            cur.pop();
            cur.pop();
            cur.pop();
        },
        _ => (),
    }
    if pos == s.len() - 2{
        return;
    }
    // 100~255
    match (s[pos],s[pos+1],s[pos+2]){
        (b'1', b'0'..=b'9', b'0'..=b'9') | 
        (b'2', b'0'..=b'4', b'0'..=b'9') |
        (b'2', b'5', b'0'..=b'5') => {
            cur.push(s[pos]);
            cur.push(s[pos+1]);
            cur.push(s[pos+2]);
            cur.push(b'.');
            backtrack(ans,s,cur,pos+3,dot_num+1);
            cur.pop();
            cur.pop();
            cur.pop();
            cur.pop();
        },
        _ => (),
    }
}