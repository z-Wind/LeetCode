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
        for num in set.iter() {
            if !set.contains(&(num-1)){
                let mut cur_num = *num;
                let mut count = 1;
                while set.contains(&(cur_num+1)){
                    cur_num += 1;
                    count += 1;
                }
                max_count = max_count.max(count);
            }
        }
        max_count
    }
}