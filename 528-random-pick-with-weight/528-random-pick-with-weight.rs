use rand::{thread_rng, Rng, rngs::ThreadRng};

struct Solution {
    sum: i32,
    w: Vec<i32>,
}


/** 
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl Solution {

    fn new(w: Vec<i32>) -> Self {
        Self {
            sum: w.iter().sum(),
            w,
        }
    }
    
    fn pick_index(&self) -> i32 {
        let mut rng = rand::thread_rng();
        let stop = rng.gen_range(0, self.sum);
        let mut sum = 0;
        for (i, num) in self.w.iter().enumerate() {
            sum += num;
            // [0, sum1), [sum1, sum2), .... [sum_n, sum)
            if sum > stop {
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