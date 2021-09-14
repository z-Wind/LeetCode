impl Solution {
    pub fn maximum_gap(mut nums: Vec<i32>) -> i32 {
        if nums.len() < 2{
            return 0;
        }
        nums.sort_unstable();
        let mut max_gap = i32::MIN;
        for w in nums.windows(2){
            max_gap = max_gap.max(w[1]-w[0]);
        }
        max_gap
    }
}