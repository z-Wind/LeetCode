//like  (1,2,3), (1,3,2), (2,1,3), (2,3,1), (3,1,2) and (3,2,1)
// 兩兩比較，若已排序，則選中最小但比前面大的，互相交換，再兩兩比較
//like (1,2,3,4), (1,2,4,3), (1,3,2,4), (1,3,4,2), (1,4,2,3), (1,4,3,2), 
impl Solution {
    pub fn next_permutation(nums: &mut Vec<i32>) {
        if !next_permutation(nums){
            nums.reverse();
        }
    }
}

fn next_permutation(nums: &mut [i32]) -> bool {
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