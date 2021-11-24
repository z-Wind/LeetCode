// https://leetcode.com/problems/perfect-rectangle/discuss/87181/Really-Easy-Understanding-Solution(O(n)-Java)

use std::collections::HashSet;
impl Solution {
    pub fn is_rectangle_cover(rectangles: Vec<Vec<i32>>) -> bool {
        let mut x = i32::MAX;
        let mut y = i32::MAX;
        let mut a = i32::MIN;
        let mut b = i32::MIN;
        let mut area = 0;

        let mut set: HashSet<(i32, i32)> = HashSet::new();
        for rec in rectangles {
            x = x.min(rec[0]);
            y = y.min(rec[1]);
            a = a.max(rec[2]);
            b = b.max(rec[3]);

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
        set.len() == 4
            && set.contains(&(x, y))
            && set.contains(&(a, y))
            && set.contains(&(x, b))
            && set.contains(&(a, b))
            && area == (a - x) * (b - y)
    }
}
