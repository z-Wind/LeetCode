impl Solution {
    pub fn find_pairs(mut nums: Vec<i32>, k: i32) -> i32 {
        nums.sort_unstable();

        let mut count = 0;
        let mut cur_num = None;
        for i in 0..nums.len() {
            if let Some(x) = cur_num {
                if x == nums[i] {
                    continue;
                }
            }
            for j in i + 1..nums.len() {
                let diff = nums[j] - nums[i];
                if diff == k {
                    count += 1;
                    break;
                } else if diff > k {
                    break;
                }
            }
            cur_num = Some(nums[i]);
        }

        return count;
    }
}
