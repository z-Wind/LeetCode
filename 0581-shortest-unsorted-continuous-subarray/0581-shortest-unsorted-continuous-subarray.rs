// https://leetcode.com/problems/shortest-unsorted-continuous-subarray/solutions/1188866/rust-two-pass-time-o-n-space-o-1/

impl Solution {
    pub fn find_unsorted_subarray(nums: Vec<i32>) -> i32 {
        let n = nums.len();

        let (left, _) = nums.iter().enumerate().rev().fold(
            (n - 1, i32::MAX),
            |(mut left, mut min), (i, &num)| {
                if num <= min {
                    min = num;
                } else {
                    left = i;
                }

                (left, min)
            },
        );

        if left == n - 1 {
            return 0;
        }

        let (right, _) =
            nums.iter()
                .enumerate()
                .fold((0, i32::MIN), |(mut right, mut max), (i, &num)| {
                    if num >= max {
                        max = num;
                    } else {
                        right = i;
                    }

                    (right, max)
                });

        // println!("{},{}", left, right);
        (right - left + 1) as i32
    }
}
