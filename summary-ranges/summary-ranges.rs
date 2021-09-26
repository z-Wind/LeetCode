impl Solution {
    pub fn summary_ranges(mut nums: Vec<i32>) -> Vec<String> {
        if nums.is_empty(){
            return vec![];
        }
        
        let mut result:Vec<String> = Vec::new();
        let mut start = None;
        for w in nums.windows(2){
            match start{
                None => {
                    if w[1] != w[0]+1{
                        result.push(format!("{}",w[0]));
                        start = Some(w[1]);
                    } else {
                        start = Some(w[0]);
                    }
                },
                Some(x) => {
                    if w[1] != w[0]+1{
                        if x == w[0]{
                            result.push(format!("{}",x));    
                        } else {
                            result.push(format!("{}->{}",x,w[0]));
                        }
                        start = Some(w[1]);
                    }
                }
            }
        }
        match start{
            Some(x) if x == nums[nums.len()-1] => result.push(format!("{}",x)),
            Some(x) if x != nums[nums.len()-1] => result.push(format!("{}->{}",x,nums[nums.len()-1])),
            None => result.push(format!("{}",nums[nums.len()-1])),
            _ => panic!(),
        }
        
        result
    }
}