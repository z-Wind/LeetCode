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
            let mut rowsum  = vec![0;row];
            for r in l..col {
                for i in 0..row {
                    rowsum[i] += matrix[i][r];
                }

                // Find the max subarray no more than K 
                let mut accuSet = BTreeSet::new();
                accuSet.insert(0);
                let mut curSum = 0;
                for sum in &rowsum{
                    curSum += sum;
                    if let Some(x) = accuSet.range(curSum - k..).next() {
                        res = res.max(curSum - x);
                        if res == k {
                            return k;
                        }
                    };
                    accuSet.insert(curSum);
                }
            }
        }
        res
    }
}