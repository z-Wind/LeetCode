use std::collections::BTreeMap;

impl Solution {
    pub fn median_sliding_window(nums: Vec<i32>, k: i32) -> Vec<f64> {
        let k = k as usize;
        let mut map:BTreeMap<i32,usize> = BTreeMap::new();
        let mut ans = Vec::new();
        for i in 0..k{
            *map.entry(nums[i]).or_insert(0) += 1;
        }
        ans.push(get_median(&map, k));
        for i in 1..nums.len()-(k-1){
            if let Some(x) = map.remove(&nums[i-1]){
                if x > 1 {
                    map.insert(nums[i-1], x-1);
                }
            }
            *map.entry(nums[i+k-1]).or_insert(0) += 1;
            
            ans.push(get_median(&map, k));
        }
        
        ans
    }
}

fn get_median(map: &BTreeMap<i32,usize>, k:usize) -> f64{
    // println!("{:?}", map);
    let mut counts = 0;
    if k % 2 == 1 {
        let mid = k/2 + 1;
        for (&num, count) in map.iter() {
            counts += count;
            if counts >= mid{
                return num as f64;
            }
        }
    } else {
        let mid1 = k/2;
        let mid2 = k/2 + 1;
        let mut median = None;
        for (&num, count) in map.iter() {
            counts += count;
            if median.is_none() && counts >= mid1 {
                median = Some(num as f64);
            }
            if median.is_some() && counts >= mid2 {
                return (median.unwrap() + num as f64) / 2.0;
            }
        }
    }
    panic!("no answer");
}