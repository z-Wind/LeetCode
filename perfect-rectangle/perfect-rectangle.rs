// https://leetcode.com/problems/perfect-rectangle/discuss/87181/Really-Easy-Understanding-Solution(O(n)-Java)

use std::collections::HashSet;
impl Solution {
    pub fn is_rectangle_cover(rectangles: Vec<Vec<i32>>) -> bool {
        let mut area = 0;

        let mut set: HashSet<(i32, i32)> = HashSet::new();
        for rec in rectangles {
            area += (rec[2] - rec[0]) * (rec[3] - rec[1]);

            let bottom_left = (rec[0], rec[1]);
            if !set.insert(bottom_left) {
                set.remove(&bottom_left);
            }
            let bottom_right = (rec[2], rec[1]);
            if !set.insert(bottom_right) {
                set.remove(&bottom_right);
            }
            let top_left = (rec[0], rec[3]);
            if !set.insert(top_left) {
                set.remove(&top_left);
            }
            let top_right = (rec[2], rec[3]);
            if !set.insert(top_right) {
                set.remove(&top_right);
            }
        }
        // println!("{:?}", set);
        if set.len() != 4{
            return false;
        }
           
        
        let mut x = i32::MAX;
        let mut y = i32::MAX;
        let mut a = i32::MIN;
        let mut b = i32::MIN;
        for p in set{
            x = x.min(p.0);
            y = y.min(p.1);
            a = a.max(p.0);
            b = b.max(p.1);
        }
        
        area == (a - x) * (b - y)
    }
}
