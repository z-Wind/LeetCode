impl Solution {
    pub fn find_unsorted_subarray(nums: Vec<i32>) -> i32 {
        let mut sorted = nums.clone();
        sorted.sort_unstable();

        let mut left: i32 = -1;
        let mut right: i32 = -1;

        for (i, (x, y)) in nums.iter().zip(sorted.iter()).enumerate() {
            if x != y {
                left = i as i32;
                break;
            }
        }
        if left == -1 {
            return 0;
        }

        for (i, (x, y)) in nums.iter().zip(sorted.iter()).enumerate().rev() {
            if x != y {
                right = i as i32;
                break;
            }
        }

        right - left + 1
    }
}
