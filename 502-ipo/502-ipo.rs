// https://leetcode.com/problems/ipo/discuss/98216/Python-Priority-Queue-with-Explanation

use std::collections::BinaryHeap;

impl Solution {
    pub fn find_maximized_capital(k: i32, mut w: i32, profits: Vec<i32>, capital: Vec<i32>) -> i32 {
        let mut pqCap: Vec<(i32, i32)> = capital
            .into_iter()
            .zip(profits.into_iter())
            .collect();
        pqCap.sort_unstable();
        let mut pqPro = BinaryHeap::new();
        
        let n = pqCap.len();
        let mut i = 0;
        for _ in 0..k {
            while i < n && pqCap[i].0 <= w {
                pqPro.push(pqCap[i].1);
                i += 1;
            }

            match pqPro.pop() {
                None => break,
                Some(pro) => w += pro,
            }
        }

        w
    }
}
