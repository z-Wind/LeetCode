// https://leetcode.com/problems/ipo/discuss/98210/Very-Simple-(Greedy)-Java-Solution-using-two-PriorityQueues

use std::cmp::Reverse;
use std::collections::BinaryHeap;

impl Solution {
    pub fn find_maximized_capital(k: i32, mut w: i32, profits: Vec<i32>, capital: Vec<i32>) -> i32 {
        let mut pqCap: BinaryHeap<Reverse<(i32, i32)>> = capital
            .into_iter()
            .zip(profits.into_iter())
            .map(|ele| Reverse(ele))
            .collect();
        let mut pqPro = BinaryHeap::new();

        for _ in 0..k {
            while let Some(Reverse((cap, pro))) = pqCap.peek() {
                if *cap > w {
                    break;
                }
                pqPro.push(*pro);
                pqCap.pop();
            }

            match pqPro.pop() {
                None => break,
                Some(pro) => w += pro,
            }
        }

        w
    }
}
