use std::mem::swap;
impl Solution {
    pub fn max_product(nums: Vec<i32>) -> i32 {
        // store the result that is the max we have found so far
        let mut r = nums[0];

        // imax/imin stores the max/min product of
        // subarray that ends with the current number A[i]
        let mut imax = r;
        let mut imin = r;
        for i in (1..nums.len()) {
            // multiplied by a negative makes big number smaller, small number bigger
            // so we redefine the extremums by swapping them
            if (nums[i] < 0) {
                swap(&mut imax, &mut imin);
            }

            // max/min product for the current number is either the current number itself
            // or the max/min by the previous number times the current one
            imax = nums[i].max(imax * nums[i]);
            imin = nums[i].min(imin * nums[i]);

            // the newly computed max value is a candidate for our global result
            r = r.max(imax);
        }
        r
    }
}