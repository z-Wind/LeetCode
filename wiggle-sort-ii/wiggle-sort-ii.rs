use std::collections::VecDeque;
use std::iter::FromIterator;
impl Solution {
    pub fn wiggle_sort(nums: &mut Vec<i32>) {  
        let mut vdq = VecDeque::from_iter(nums.clone());
        for _ in 0..nums.len(){
            let num = vdq.pop_front().unwrap();
            match wiggle_sort(&mut vdq, num, true){
                None => (),
                Some(mut x) => {
                    x.push(num);
                    x.reverse();
                    *nums = x;
                    return;
                }
            }
            vdq.push_back(num);
        }
    }
}

fn wiggle_sort(nums: &mut VecDeque<i32>, prev:i32, odd:bool) -> Option<Vec<i32>>{
    // println!("{},{}, {:?}", num, odd, nums);
    if nums.is_empty(){
        return Some(vec![]);
    }
    for i in 0..nums.len(){
        if (odd && nums[i] > prev) || (!odd && nums[i] < prev){
            let cur = nums.swap_remove_front(i).unwrap();
            match wiggle_sort(nums, cur, !odd){
                None => (),
                Some(mut x) => {
                    x.push(cur);
                    return Some(x);
                }
            }
            nums.push_front(cur);
        }
    }
    None
}