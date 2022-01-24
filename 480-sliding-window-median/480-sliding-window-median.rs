// https://leetcode.com/problems/sliding-window-median/discuss/96346/Java-using-two-Tree-Sets-O(n-logk)/538502

use std::cmp::Reverse;
use std::collections::BTreeSet;

impl Solution {
    pub fn median_sliding_window(nums: Vec<i32>, k: i32) -> Vec<f64> {
        let k = k as usize;
        let n = nums.len();
        let mut left = BTreeSet::new();
        let mut right = BTreeSet::new();
        let mut res = vec![0.0; n - k + 1];

        for i in 0..k {
            left.insert(Reverse((nums[i], i)));
        }
        balance(&mut left, &mut right);
        res[0] = getMedian(k, &left, &right);

        let mut r = 1;
        for i in k..n {
            if !left.remove(&Reverse((nums[i - k], i - k))) {
                right.remove(&(nums[i - k], i - k));
            }

            right.insert((nums[i], i));
            let e = right.iter().next().unwrap().clone();
            right.remove(&e);
            left.insert(Reverse(e));
            balance(&mut left, &mut right);
            
            res[r] = getMedian(k, &left, &right);
            r += 1;
        }

        return res;
    }
}

fn balance(left: &mut BTreeSet<Reverse<(i32, usize)>>, right: &mut BTreeSet<(i32, usize)>) {
    while left.len() > right.len() {
        let e = left.iter().next().unwrap().clone();
        left.remove(&e);
        right.insert(e.0);
    }
}

fn getMedian(k: usize, left: &BTreeSet<Reverse<(i32, usize)>>, right: &BTreeSet<(i32, usize)>) -> f64 {
    // k % 2 == 0
    if k & 1 == 0 {
        return ((left.iter().next().unwrap().0).0 as f64 + right.iter().next().unwrap().0 as f64) / 2.0;
    } else {
        return right.iter().next().unwrap().0 as f64;
    }
}