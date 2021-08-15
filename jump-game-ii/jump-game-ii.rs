use std::collections::LinkedList;

impl Solution {
    pub fn jump(mut nums: Vec<i32>) -> i32 {
        let mut queue: LinkedList<(usize,usize)> = LinkedList::new();
        let mut i = 0;
        let mut level = 0;
        while i != nums.len()-1{
            //println!("{:?}", nums);
            for step in (1..=nums[i] as usize){
                if i+step < nums.len(){
                    queue.push_back((level+1,i+step));
                }
                let (index,overflow) = i.overflowing_sub(step);
                if !overflow{
                    queue.push_back((level+1,index));
                }
            }
            nums[i] = -nums[i];
            while !nums[i].is_positive() && i != nums.len()-1{
                match queue.pop_front(){
                    None => return -1,
                    Some((l,index)) => {
                        level = l;
                        i = index;
                    }
                }
            }
        }
        level as i32
    }
}