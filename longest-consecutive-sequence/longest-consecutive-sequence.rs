use std::collections::HashSet;
impl Solution {
    pub fn longest_consecutive(mut nums: Vec<i32>) -> i32 {
        if nums.is_empty(){
            return 0;
        }
        
        let mut set:HashSet<i32> = HashSet::new();
        for num in nums{
            set.insert(num);
        }
        // println!("{:?}", set);
        let mut max_count = 1;
        while !set.is_empty(){
            let mut count = 1;
            let num = set.iter().next().unwrap().clone();
            set.remove(&num);
            
            let mut num_add = num + 1;
            while let Some(_) = set.take(&num_add){
                count+=1;
                num_add+=1;
            }
            
            let mut num_sub = num - 1;
            while let Some(_) = set.take(&num_sub){
                count+=1;
                num_sub-=1;
            }
            
            max_count = max_count.max(count);
            // println!("{}: {:?}",num, set);
        }
        max_count
    }
}