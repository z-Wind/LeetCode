// by Solution
// Approach #2 Dynamic Programming [Accepted]
// Weak arithmetic subsequences are subsequences that consist of at least two elements and if the difference between any two consecutive elements is the same.
// There are two properties of weak arithmetic subsequences that are very useful:
//      1. For any pair i, j (i != j), A[i] and A[j] can always form a weak arithmetic subsequence.
//      2. If we can append a new element to a weak arithmetic subsequence and keep it arithmetic, then the new subsequence must be an arithmetic subsequence.
// f[i][d] denotes the number of weak arithmetic subsequences that ends with A[i] and its common difference is d.
// for all j < i, f[i][A[i] - A[j]] += (f[j][A[i] - A[j]] + 1).
// The 1 appears here because of the property one, we can form a new weak arithmetic subsequence for the pair (i, j).

use std::collections::HashMap;
impl Solution {
    pub fn number_of_arithmetic_slices(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        let mut ans = 0;
        let mut cnt:Vec<HashMap<i64,i32>> = vec![HashMap::new();n];
        for i in 0..n {
            for j in 0..i {
                let delta = nums[i] as i64 - nums[j] as i64;
                let sum = *cnt[j].get(&delta).unwrap_or(&0);
                *cnt[i].entry(delta).or_insert(0) += sum + 1;
                // sum means the number of at least 2 elements, and append nums[i] to greater or equal to 3 elements
                ans += sum;
            }
        }
        return ans;       
    }
}