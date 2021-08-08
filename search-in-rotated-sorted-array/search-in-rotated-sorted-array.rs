impl Solution {
    pub fn search(mut nums: Vec<i32>, target: i32) -> i32 {
        match nums.iter().position(|&x| x == target) {
            Some(i) => i as i32,
            None => -1,
        }
    }
}