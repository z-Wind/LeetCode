use std::collections::BTreeMap;
impl Solution {
    pub fn max_sliding_window(nums: Vec<i32>, k: i32) -> Vec<i32> {
        let k = k as usize;
        if nums.len() < k{
            return vec![nums.into_iter().max().unwrap()];
        }
        
        let mut bt:BTreeMap<i32,i32> = BTreeMap::new();
        nums[0..k].iter().for_each(|&x| *bt.entry(x).or_insert(0)+=1);
        let mut v:Vec<i32> = vec![*bt.keys().last().unwrap()];
        for i in (k..nums.len()){
            *bt.entry(nums[i]).or_insert(0) +=1;
            let x = bt.get_mut(&nums[i-k]).unwrap();
            *x -= 1;
            if *x == 0{
                bt.remove(&nums[i-k]);
            }
            v.push(*bt.keys().last().unwrap());
        }
        v
    }
}