use std::collections::HashSet;

impl Solution {
    pub fn find_pairs(nums: Vec<i32>, k: i32) -> i32 {
        let mut set = HashSet::new();
        let mut record = HashSet::new();

        let mut count = 0;
        for num in nums {
            if let Some(x) = num.checked_add(k) {
                if set.contains(&x) && !record.contains(&(num, x)){
                    record.insert((num, x));
                    count += 1;
                }
            }
            if let Some(x) = num.checked_sub(k) {
                if set.contains(&x) && !record.contains(&(x, num)){
                    record.insert((x, num));
                    count += 1;
                }
            }
            
            set.insert(num);
        }

        return count;
    }
}
