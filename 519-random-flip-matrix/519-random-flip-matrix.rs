use rand::seq::SliceRandom;
use rand::thread_rng;

struct Solution {
    m: i32,
    n: i32,
    ij: Vec<Vec<i32>>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl Solution {
    fn init(m: i32, n: i32) -> Vec<Vec<i32>> {
        let mut rng = thread_rng();
        let mut i:Vec<i32> = (0..m).collect();
        i.shuffle(&mut rng);
        let mut j:Vec<i32> = (0..n).collect();
        j.shuffle(&mut rng);
        
        let mut ij = Vec::with_capacity(1000);
        for x in i {
            for &y in j.iter() {
                ij.push(vec![x, y]);
                if ij.len() > 1000 {
                    return ij;
                }
            }
        }
        return ij;
    }
    
    fn new(m: i32, n: i32) -> Self {
        let ij = Self::init(m, n);
        // println!("{:?}", ij);
        Self {
            m,
            n,
            ij,
        }
    }

    fn flip(&mut self) -> Vec<i32> {
        self.ij.pop().unwrap()
    }

    fn reset(&mut self) {
        self.ij = Solution::init(self.m, self.n);
    }
}

/**
 * Your Solution object will be instantiated and called as such:
 * let obj = Solution::new(m, n);
 * let ret_1: Vec<i32> = obj.flip();
 * obj.reset();
 */