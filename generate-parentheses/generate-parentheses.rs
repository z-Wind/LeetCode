fn generate_parenthesis(s:String, par_nums:i32, n: i32) -> Vec<String> {
    let mut ans:Vec<String> = vec![];
    match(par_nums, n){
        (0,0) => return vec![s],
        (_,0) => ans.append(&mut generate_parenthesis(format!("{})",s), par_nums-1, n)),
        (0,_) => ans.append(&mut generate_parenthesis(format!("{}(",s), par_nums+1, n-1)),
        (_,_) => {
            ans.append(&mut generate_parenthesis(format!("{}(",s), par_nums+1, n-1));
            ans.append(&mut generate_parenthesis(format!("{})",s), par_nums-1, n));
        },
    }
    
    ans
}


impl Solution {
    pub fn generate_parenthesis(n: i32) -> Vec<String> {
        generate_parenthesis(String::from("("), 1, n-1)
    }
}