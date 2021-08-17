use std::char;
impl Solution {
    pub fn get_permutation(n: i32, k: i32) -> String {
        let mut first:Vec<u32> = (1..=n as u32).collect();
        if k == 1{
            return first.into_iter().map(|x| char::from_digit(x, 10).unwrap()).collect();
        }
        
        for _ in (2..=k){
            next_permutation(&mut first);
        }
        
        first.into_iter().map(|x| char::from_digit(x, 10).unwrap()).collect()
    }
}

fn next_permutation(nums: &mut [u32]) -> bool {
    //println!("{:?}",nums);
    if nums.len() == 1 {
        return false;
    }
    
    if next_permutation(&mut nums[1..]) {
        return true;
    }
    
    if nums[0] >= nums[1] {
        return false;
    }
    
    nums[1..].reverse();
    //println!("reverse: {:?}",nums);
    let mut swap_i = 0;
    for (i,v) in nums[1..].iter().enumerate(){
        if *v > nums[0]{
            swap_i = i+1;
            break;
        }
    }
    nums.swap(0,swap_i);
    return true;
}