use std::collections::HashSet;

impl Solution {
    pub fn find_pairs(nums: Vec<i32>, k: i32) -> i32 {
        let mut set = HashSet::new();
        let mut record = HashSet::new();

        let mut count = 0;
        for num in nums {
            let x = num + k;
            if set.contains(&x) && !record.contains(&(num, x)){
                record.insert((num, x));
                count += 1;
            }
            
            let x = num - k;
            if set.contains(&x) && !record.contains(&(x, num)){
                record.insert((x, num));
                count += 1;
            }

            set.insert(num);
        }

        return count;
    }
}
