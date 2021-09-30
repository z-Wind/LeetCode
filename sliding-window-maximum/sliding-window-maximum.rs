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
        let mut pre = nums[0];
        for w in nums[1..].windows(k){
            // println!("{}: {:?}",pre, bt);
            *bt.entry(w[k-1]).or_insert(0) +=1;
            let x = bt.get_mut(&pre).unwrap();
            *x -= 1;
            if *x == 0{
                bt.remove(&pre);
            }
            v.push(*bt.keys().last().unwrap());
            pre = w[0];
        }
        v
    }
}