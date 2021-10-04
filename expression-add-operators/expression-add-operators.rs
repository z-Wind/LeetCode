impl Solution {
    pub fn add_operators(num: String, target: i32) -> Vec<String> {
        let mut rst = Vec::new();
        if num.is_empty() {
            return rst
        }
        
        helper(&mut rst, String::new(), num.as_bytes(), target as i64, 0, 0, 0);
        rst
    }
}

fn helper(rst:&mut Vec<String>, path:String, num:&[u8], target:i64, pos:usize, eval:i64, multed:i64){
    if(pos == num.len()){
        if(target == eval){
            rst.push(path);
        }
        return;
    }
    let mut cur:i64 = 0;
    for i in (pos..num.len()){
        if i != pos && num[pos] == b'0'{
            break;  
        } 
        cur = 10*cur + (num[i] - b'0') as i64; 
        if pos == 0 {
            helper(rst, format!("{}{}", path, cur), num, target, i + 1, cur, cur);
        }
        else{
            helper(rst, format!("{}+{}", path, cur), num, target, i + 1, eval + cur , cur);

            helper(rst, format!("{}-{}", path, cur), num, target, i + 1, eval - cur, -cur);

            helper(rst, format!("{}*{}", path, cur), num, target, i + 1, eval - multed + multed * cur, multed * cur );
        }
    }
}