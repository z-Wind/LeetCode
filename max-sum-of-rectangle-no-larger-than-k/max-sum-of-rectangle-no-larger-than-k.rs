// https://leetcode.com/problems/max-sum-of-rectangle-no-larger-than-k/discuss/83599/Accepted-C%2B%2B-codes-with-explanation-and-references
use std::collections::BTreeSet;
impl Solution {
    pub fn max_sum_submatrix(matrix: Vec<Vec<i32>>, k: i32) -> i32 {
        if matrix.is_empty(){ 
            return 0;
        }
        let row = matrix.len();
        let col = matrix[0].len();
        let mut res = i32::MIN;
        for l in 0..col {
            let mut sums = vec![0;row];
            for r in l..col {
                for i in 0..row {
                    sums[i] += matrix[i][r];
                }

                // Find the max subarray no more than K 
                let mut accuSet = BTreeSet::new();
                accuSet.insert(0);
                let mut curSum = 0;
                let mut curMax = i32::MIN;
                for &sum in sums.iter(){
                    curSum += sum;
                    let iter = accuSet.range(curSum - k..);
                    match iter.map(|x| curSum-x).max(){
                        Some(x) => curMax = curMax.max(x),
                        _ => (),
                    }
                    accuSet.insert(curSum);
                }
                res = res.max(curMax);
            }
        }
        return res;
    }
}