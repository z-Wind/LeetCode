// https://leetcode.com/problems/poor-pigs/discuss/94266/Another-explanation-and-solution
// With 2 pigs, poison killing in 15 minutes, and having 60 minutes, 25 buckets 
//  1  2  3  4  5
//  6  7  8  9 10
// 11 12 13 14 15
// 16 17 18 19 20
// 21 22 23 24 25
// one pig to find the row (make it drink from buckets 1, 2, 3, 4, 5, wait 15 minutes, make it drink from buckets 6, 7, 8, 9, 10, wait 15 minutes, etc). 
// second pig to find the column (make it drink 1, 6, 11, 16, 21, then 2, 7, 12, 17, 22, etc).
// (x,y)
// With 3 pigs, we can similarly use a 5×5×5 cube instead of a 5×5 square and again use one pig to determine the coordinate of one dimension 
// (x,y,z)
// one pig means one coordinate
// tests means the numbers of the coordinate

impl Solution {
    pub fn poor_pigs(buckets: i32, minutes_to_die: i32, minutes_to_test: i32) -> i32 {
        let mut m = 1 + minutes_to_test/minutes_to_die;
        let mut pigs = (buckets as f64).log(m as f64).ceil() as i32;
        pigs
    }
}