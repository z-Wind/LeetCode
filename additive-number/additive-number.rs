impl Solution {
    pub fn is_additive_number(num: String) -> bool {
        if num.len() < 3{
            return false;
        }
        
        let nums = num.as_bytes();
        let mut add1 = 0;
        for i in (0..nums.len()-2){
            add1 = (add1*10 + (nums[i]-b'0') as i64);
            let mut add2 = 0;
            for j in (i+1..nums.len()-1){
                add2 = (add2*10 + (nums[j]-b'0') as i64);
                if is_additive_number(&nums[j+1..], add1, add2){
                    return true;
                }
                if add2 == 0{
                    break;
                }
            }
            if add1 == 0{
                break;
            }
        }
        false
    }
}

fn is_additive_number(nums: &[u8], add1:i64, add2:i64) -> bool {
    // println!("{}+{}: {:?}", add1, add2, nums.iter().map(|x| x-48).collect::<Vec::<u8>>());
    if nums.len() == 0{
        return true;
    }
    
    let mut result = 0;   
    let ans = add1 + add2;
    for i in (0..nums.len()){
        result = (result*10 + (nums[i]-b'0') as i64);
        if result == ans{
            return is_additive_number(&nums[i+1..], add2, result);
        } else if result > ans{
            break;
        }
        if result == 0{
            break;
        }
    }
    false
}