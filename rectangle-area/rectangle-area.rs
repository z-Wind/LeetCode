impl Solution {
    pub fn compute_area(ax1: i32, ay1: i32, ax2: i32, ay2: i32, bx1: i32, by1: i32, bx2: i32, by2: i32) -> i32 {
        let a_area = area(ax1, ay1, ax2, ay2);
        let b_area = area(bx1, by1, bx2, by2);
        let overlap = area(ax1.max(bx1), ay1.max(by1), ax2.min(bx2), ay2.min(by2));        
        
        a_area + b_area - overlap
    }
}

fn area(ax1: i32, ay1: i32, ax2: i32, ay2: i32) -> i32{
    // println!("({},{}) ({},{})",ax1,ay1,ax2,ay2);
    if ax1 > ax2 || ay1 > ay2{
        return 0;
    }
    ((ax2-ax1)*(ay2-ay1)).abs()
}