use std::collections::LinkedList;

impl Solution {
    pub fn jump(mut nums: Vec<i32>) -> i32 {
        let mut queue: LinkedList<(usize,usize)> = LinkedList::new();
        let mut i = 0;
        let mut level = 0;
        'outer: while i != nums.len()-1{
            //println!("{:?}", nums);  
            if nums[i].is_positive(){
                for step in (1..=nums[i] as usize){
                    if i+step == nums.len()-1{
                        level += 1;
                        break 'outer;
                    } else if i+step < nums.len() && nums[i+step].is_positive(){
                        //println!("push {}", i+step);
                        queue.push_back((level+1,i+step));
                    }
                    let (index,overflow) = i.overflowing_sub(step);
                    if !overflow && nums[index].is_positive(){
                        //println!("push {}", index);
                        queue.push_back((level+1,index));
                    }
                }
                nums[i] = -nums[i];   
            }
            
            match queue.pop_front(){
                None => return -1,
                Some((l,index)) => {
                    level = l;
                    i = index;
                }
            }
            //println!("pop:{:?}", i);            
        }
        level as i32
    }
}