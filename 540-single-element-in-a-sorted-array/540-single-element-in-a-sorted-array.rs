impl Solution {
    pub fn single_non_duplicate(nums: Vec<i32>) -> i32 {
        let mut left = 0;
        let mut right = nums.len() - 1;
        while right > left {
            let mid = left + (right - left) / 2;
            // println!("{},{},{}", nums[left], nums[mid], nums[right]);

            // 1,1,3,3,4
            // 0,1,1,3,3
            // 1,1,2,3,3
            if (mid - left) % 2 == 0 {
                if nums[mid] == nums[mid + 1] {
                    left = mid + 2;
                } else if nums[mid] == nums[mid - 1] {
                    right = mid - 2;
                } else {
                    return nums[mid];
                }
            }
            // 1,1,3,3,4,4,5
            // 0,1,1,3,3,4,4
            else {
                if nums[mid] == nums[mid - 1] {
                    left = mid + 1;
                } else {
                    right = mid - 1;
                }
            }
        }
        nums[left]
    }
}