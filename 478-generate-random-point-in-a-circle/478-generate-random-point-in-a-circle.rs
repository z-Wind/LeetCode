// https://leetcode.com/problems/generate-random-point-in-a-circle/discuss/1113679/Python-Polar-coordinates-explained-with-diagrams-and-math
// https://leetcode.com/problems/generate-random-point-in-a-circle/discuss/1113679/Python-Polar-coordinates-explained-with-diagrams-and-math/879669

use rand::{thread_rng, Rng};

struct Solution {
    r: f64,
    x: f64,
    y: f64,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl Solution {
    fn new(radius: f64, x_center: f64, y_center: f64) -> Self {
        Self {
            r: radius,
            x: x_center,
            y: y_center,
        }
    }

    fn rand_point(&self) -> Vec<f64> {
        let mut rng = rand::thread_rng();
        let pi = std::f64::consts::PI;
        let theta = 2.0 * pi * rng.gen::<f64>();

        // Uniformly pick a random area between 0 and maximum possible area
        // area = pi * self.r * self.r
        // R = (rng.gen_range(0.0, area) / pi).sqrt()
        // omit pi
        let area = self.r * self.r;
        let r = rng.gen_range(0.0, area).sqrt();

        vec![self.x + r * theta.cos(), self.y + r * theta.sin()]
    }
}

/**
 * Your Solution object will be instantiated and called as such:
 * let obj = Solution::new(radius, x_center, y_center);
 * let ret_1: Vec<f64> = obj.rand_point();
 */