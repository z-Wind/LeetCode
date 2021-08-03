fn combine(v:Vec<String>, s:Vec<String>) -> Vec<String>{
    if v.len() == 0{
        return s;
    }
    
    let mut result:Vec<String> = vec![];
    for c1 in v.iter(){
        for c2 in s.iter(){
            result.push(format!("{}{}",c1,c2));
        }
    }
    
    result
}

impl Solution {
    pub fn letter_combinations(digits: String) -> Vec<String> {
        let mut ans:Vec<String> = vec![];
        if digits == ""{
            return ans;
        }
        
        for num in digits.chars(){
            match num{
                '2' => ans = combine(ans, ('a' ..= 'c').map(|c| c.to_string()).collect()), 
                '3' => ans = combine(ans, ('d' ..= 'f').map(|c| c.to_string()).collect()), 
                '4' => ans = combine(ans, ('g' ..= 'i').map(|c| c.to_string()).collect()), 
                '5' => ans = combine(ans, ('j' ..= 'l').map(|c| c.to_string()).collect()), 
                '6' => ans = combine(ans, ('m' ..= 'o').map(|c| c.to_string()).collect()), 
                '7' => ans = combine(ans, ('p' ..= 's').map(|c| c.to_string()).collect()), 
                '8' => ans = combine(ans, ('t' ..= 'v').map(|c| c.to_string()).collect()), 
                '9' => ans = combine(ans, ('w' ..= 'z').map(|c| c.to_string()).collect()), 
                _ => panic!(),
            }
        }
      
        ans
    }
}