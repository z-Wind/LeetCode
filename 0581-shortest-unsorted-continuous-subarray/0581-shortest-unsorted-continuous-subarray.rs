impl Solution {
    pub fn find_unsorted_subarray(nums: Vec<i32>) -> i32 {
        let n = nums.len();

        let mut left = nums
            .windows(2)
            .enumerate()
            .find(|(_, w)| w[0] > w[1])
            .map(|(i, _)| i as i32)
            .unwrap_or(-1);
        if left == -1 {
            return 0;
        }
        let minimum = nums[left as usize + 1..].iter().min().cloned().unwrap();

        left = (0..left + 1)
            .rev()
            .find(|&i| nums[i as usize] <= minimum)
            .unwrap_or(-1);

        let mut right = nums
            .windows(2)
            .enumerate()
            .rev()
            .find(|(_, w)| w[0] > w[1])
            .map(|(i, _)| i as i32)
            .unwrap_or(-1);

        let maxmum = nums[..=right as usize].iter().max().cloned().unwrap();

        right = (right + 1..n as i32)
            .find(|&i| nums[i as usize] >= maxmum)
            .unwrap_or(n as i32);

        // println!("{},{}", left, right);
        right - left - 1
    }
}
