use rand::{thread_rng, Rng};

struct Solution {
    r: f64, 
    x: f64, 
    y: f64
}


/** 
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl Solution {

    fn new(radius: f64, x_center: f64, y_center: f64) -> Self {
        Self{
            r: radius,
            x: x_center,
            y: y_center,
        }   
    }
    
    fn rand_point(&self) -> Vec<f64> {
        let mut rng = rand::thread_rng();
        loop {
            let x = 2.0 * rng.gen::<f64>() - 1.0;
            let y = 2.0 * rng.gen::<f64>() - 1.0;
            
            if x*x + y*y <= 1.0{
                return vec![self.r * x + self.x, self.r * y + self.y];
            }
        }
    }
}

/**
 * Your Solution object will be instantiated and called as such:
 * let obj = Solution::new(radius, x_center, y_center);
 * let ret_1: Vec<f64> = obj.rand_point();
 */