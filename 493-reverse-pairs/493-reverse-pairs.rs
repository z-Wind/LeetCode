// https://github.com/z-Wind/LeetCode/blob/main/count-of-range-sum/count-of-range-sum.rs

impl Solution {
    pub fn reverse_pairs(nums: Vec<i32>) -> i32 {
        let mut c = CountReversePair::new();
        c.count(nums)
    }
}

struct CountReversePair {
    count: i32,
}

impl CountReversePair {
    fn new() -> Self {
        Self { count: 0 }
    }
    fn count(&mut self, nums: Vec<i32>) -> i32 {
        let mut nums: Vec<i64> = nums.into_iter().map(|x| x as i64).collect();
        let n = nums.len();
        self.mergesort(&mut nums, 0, n - 1);
        self.count
    }

    fn mergesort(&mut self, nums: &mut Vec<i64>, start: usize, end: usize) {
        if start >= end {
            return;
        }
        let mid = start + (end - start) / 2;
        self.mergesort(nums, start, mid);
        self.mergesort(nums, mid + 1, end);
        self.merge(nums, start, mid, end);
    }

    fn merge(&mut self, nums: &mut Vec<i64>, start: usize, mid: usize, end: usize) {
        // println!("merge: left {:?}, right {:?}", &nums[start..=mid], &nums[mid+1..=end]);

        let mut right = mid + 1;
        let mut merged: Vec<i64> = Vec::with_capacity(end - start + 1);

        let mut i = mid + 1;
        for left in start..=mid {
            while right <= end && nums[right] < nums[left] {
                merged.push(nums[right]);
                right += 1;
            }
            merged.push(nums[left]);

            // extra action for count
            while i <= end && nums[left] > 2 * nums[i] {
                i += 1;
            }
            self.count += (i - mid - 1) as i32;
        }
        while right <= end {
            merged.push(nums[right]);
            right += 1;
        }

        // part of normal merge sort
        // copy back merged result into array
        let mut pos = start;
        for num in merged {
            nums[pos] = num;
            pos += 1;
        }
        // println!("{}: merged: {:?}", self.count, &nums[start..=end]);
    }
}
