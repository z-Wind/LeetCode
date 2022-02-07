impl Solution {
    pub fn construct_rectangle(area: i32) -> Vec<i32> {
        let mut W = (area as f64).sqrt() as i32;
        loop {
            if area % W == 0{
                return vec![area/W, W];
            }
            W -= 1;
        }
    }
}