// https://leetcode.com/problems/random-flip-matrix/discuss/155341/Intuitive-Algorithm-Explanation-Step-By-Step.
// https://leetcode.com/problems/random-flip-matrix/discuss/155341/Intuitive-Algorithm-Explanation-Step-By-Step./558400

// For example, row = 2, cols = 3, total = 6
//
// random select from 0 1 2 3 4 5
// r = 5
// total: 5,
// x = 5
// map[5] = 5
// This case is simple. Next time the number will be generated within 0-4, so 5 will never appear again.
// The values {5}
//
// random select from 0 1 2 3 4
// r = 2
// total: 4
// x = 2
// map[5] = 5, map[2] = 4
// The values {5, 2}
//
// random select from 0 1 2 3
// r = 2, so 2 appears again,
// total: 3
// as map[2] = 4, so x = 4
// map[5] = 5, map[2] = 3(update)
// The values {5, 2, 4}
//
// random select from 0 1 2
// r = 1
// total: 2
// x = 1
// map[5] = 5, map[2] = 3, map[1] = 3(2 -> map[2])
// The values {5, 2, 4, 1}
//
// random select from 0 1
// r = 1
// total: 1
// as map[1] = 3, so x = 3
// map[5] = 5, map[2] = 3, map[1] = 3(1 -> map[1])
// The values {5, 2, 4, 1, 3}
//
// random select from 0
// r = 0
// total: 0
// x = 0
// map[5] = 5, map[2] = 3, map[1] = 3, map[0] = 0
// The values {5, 2, 4, 1, 3, 0}

use rand::{rngs::ThreadRng, thread_rng, Rng};
use std::collections::HashMap;

struct Solution {
    map: HashMap<i32, i32>,
    rows: i32,
    cols: i32,
    total: i32,
    rng: ThreadRng,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl Solution {
    fn new(m: i32, n: i32) -> Self {
        Self {
            map: HashMap::new(),
            rows: m,
            cols: n,
            total: m * n,
            rng: rand::thread_rng(),
        }
    }

    fn flip(&mut self) -> Vec<i32> {
        // generate index, decrease total number of values
        let r = self.rng.gen_range(0, self.total);
        self.total -= 1;
        // check if we have already put something at this index
        let x = self.map.get(&r).cloned().unwrap_or(r);
        // swap - put total at index that we generated
        self.map.insert(r, self.map.get(&self.total).cloned().unwrap_or(self.total));

        return vec![x / self.cols, x % self.cols];
    }

    fn reset(&mut self) {
        self.map.clear();
        self.total = self.rows * self.cols;
    }
}

/**
 * Your Solution object will be instantiated and called as such:
 * let obj = Solution::new(m, n);
 * let ret_1: Vec<i32> = obj.flip();
 * obj.reset();
 */