impl Solution {
    pub fn add_operators(num: String, target: i32) -> Vec<String> {
        let mut rst = Vec::new();
        if num.is_empty() {
            return rst
        };
        
        helper(&mut rst, String::new(), &num, target as i64, 0, 0, 0);
        rst
    }
}

fn helper(rst:&mut Vec<String>, path:String, num:&str, target:i64, pos:usize, eval:i64, multed:i64){
    if(pos == num.len()){
        if(target == eval){
            rst.push(path);
        }
        return;
    }
    for i in (pos..num.len()){
        if i != pos && &num[pos..pos+1] == "0"{
            break;  
        } 
        let cur:i64 = num[pos..i + 1].parse().unwrap();
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