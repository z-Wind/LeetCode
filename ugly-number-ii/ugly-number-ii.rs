use std::collections::BTreeSet;
impl Solution {
    pub fn nth_ugly_number(n: i32) -> i32 {
        let n = n as usize;
        let mut nums = vec![];
        let mut set:BTreeSet<usize> = BTreeSet::new();
        set.insert(1);
        while nums.len() < n{
            let num = *set.iter().next().unwrap();
            nums.push(num as i32);
            set.remove(&num);
            set.insert(num*2);
            set.insert(num*3);
            set.insert(num*5);
        }
        // println!("{:?}", nums);
        nums[n-1]
    }
}