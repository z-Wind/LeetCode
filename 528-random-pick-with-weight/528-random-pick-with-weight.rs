use rand::{thread_rng, Rng, rngs::ThreadRng};

struct Solution {
    sums: Vec<i32>,
}


/** 
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl Solution {

    fn new(w: Vec<i32>) -> Self {
        let mut sums = vec![0; w.len()+1];
        for (i, num) in w.iter().enumerate() {
            sums[i+1] = sums[i] + num;
        }
        Self {
            sums,
        }
    }
    
    fn pick_index(&self) -> i32 {
        let mut rng = rand::thread_rng();
        let stop = rng.gen_range(0, self.sums.last().unwrap());
        for (i, bound) in self.sums.windows(2).enumerate() {
            // [0, sum1), [sum1, sum2), .... [sum_n, sum_total)
            if bound[0] <= stop && stop < bound[1] {
                return i as i32;
            }
        }
        unreachable!("The loop should always return");
    }
}

/**
 * Your Solution object will be instantiated and called as such:
 * let obj = Solution::new(w);
 * let ret_1: i32 = obj.pick_index();
 */