use rand::{rngs::ThreadRng, thread_rng, Rng};

struct Solution {
    sums: Vec<i32>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl Solution {
    fn new(mut w: Vec<i32>) -> Self {
        for i in 1..w.len() {
            w[i] += w[i-1];
        }
        Self { sums:w }
    }

    fn pick_index(&self) -> i32 {
        let mut rng = rand::thread_rng();
        let stop = rng.gen_range(1, self.sums[self.sums.len() - 1] + 1);
        // [1, sum1], [sum1+1, sum2], .... [sum_n+1, sum_total]
        self.sums.binary_search(&stop).unwrap_or_else(|x| x) as i32
    }
}


/**
 * Your Solution object will be instantiated and called as such:
 * let obj = Solution::new(w);
 * let ret_1: i32 = obj.pick_index();
 */