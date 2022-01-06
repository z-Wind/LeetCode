impl Solution {
    pub fn hamming_distance(x: i32, y: i32) -> i32 {
        let diff = x ^ y;
        diff.count_ones() as i32
    }
}