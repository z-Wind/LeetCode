// https://leetcode.com/problems/sliding-window-median/discuss/1341219/Rust-Binary-Search-%2B-Vec-Heap-beats-100-mem-and-speed

impl Solution {
    pub fn median_sliding_window(nums: Vec<i32>, k: i32) -> Vec<f64> {
        // println!("{:?}", nums);
        
        let k = k as usize;
        let mut res: Vec<f64> = vec![];
        let mut window:Vec<i32> = Vec::new();
        window.extend_from_slice(&nums[..k]);
        window.sort_unstable();
        res.push(get_median(&window, k));
        
        for i in 1..nums.len()-(k-1) {
            // println!("{} {:?} {:?}", nums[i-1], window, window.binary_search(&nums[i-1]));
            // remove previous elem
            window.remove(window.binary_search(&nums[i-1]).unwrap());
            
            // find where to insert
            let num = nums[i+k-1];
            let idx = window.binary_search(&num).unwrap_or_else(|x| x);
            // add new elem
            window.insert(idx, num);
            
            // get median
            res.push(get_median(&window, k));
        }
        res
    }
}

fn get_median(window:&Vec<i32>, k:usize) -> f64{
    let mid = k / 2;
    if k % 2 == 1 {
        window[mid] as f64
    } else {
        (window[mid - 1] as f64 + window[mid] as f64) / 2.0
    }
}