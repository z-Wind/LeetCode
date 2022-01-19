// https://github.com/z-Wind/LeetCode/blob/main/461-hamming-distance/461-hamming-distance.rs

impl Solution {
    pub fn total_hamming_distance(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        let mut sum = 0;
        for i in 0..n {
            for j in i + 1..n {
                sum += hamming_distance(nums[i], nums[j]);
            }
        }
        sum
    }
}

fn hamming_distance(x: i32, y: i32) -> i32 {
    let diff = x ^ y;
    diff.count_ones() as i32
}
