// https://leetcode.com/problems/minimum-moves-to-equal-array-elements-ii/discuss/94937/Java(just-like-meeting-point-problem)/195799

// if nums is sorted
// Let's start from C is min, the total move is mi, x mean from nums[i]
// 1. If C = min + 1 and C <= nums[1], is it better or worse?
//    move = m0 + 1 - (n - 1) = m0 + (2 - n)  when n >= 2, it's better.
//    If C = min + 2 and C <= nums[1], is it better or worse?
//    move = (m0 + 2 - n) + 1 - (n - 1) = m0 + (2 - n) + (2 - n) when n >= 2, it's better
//    If C = min + 3 and C <= nums[1], is it better or worse?
//    move = (m0 + (2 - n) + (2 - n)) + 1 - (n - 1)
//         =  m0 + (1*2 - n) + (1*2 - n) + (1*2 - n) when n >= 2, it's better
//    So we know that for C, from nums[0] to nums[1], we are always getting better result whe n >= 2.
// Likewise,
// 2. If C from nums[1] to nums[2], move = m1 + 2 - (n - 2) = m1 + 4 - n
//    => m1 + (2*2 - n) + (2*2 - n) + ..., so we get better result for n >= 4
// 3. If C from nums[2] to nums[3], move = m2 + 3 - (n - 3) = m1 + 6 - n,
//    => m2 + (3*2 - n) + (3*2 - n) + ..., so we get better result for n >= 6
// so from nums[N/2-1] to num[N/2], move = m_(N/2-1) + (N/2*2 - N) + (N/2*2 - N) + ...
//                                       = m_(N/2-1) for N
// For example, [1, 3, 5, 8], C can be 3, 4, 5. All work.
//
// (C - nums[0]) + ... + (nums[n - 1] - C) = (nums[n - 1] - nums[0]) + (nums[n - 2] - nums[1]) ..

impl Solution {
    pub fn min_moves2(mut nums: Vec<i32>) -> i32 {
        nums.sort_unstable();

        let mut mov = 0;
        let mut i = 0;
        let mut j = nums.len() - 1;
        while i < j {
            mov += nums[j] - nums[i];
            i += 1;
            j -= 1;
        }
        mov
    }
}
