impl Solution {
    pub fn compute_area(ax1: i32, ay1: i32, ax2: i32, ay2: i32, bx1: i32, by1: i32, bx2: i32, by2: i32) -> i32 {
        let a_area = area(ax1, ay1, ax2, ay2);
        let b_area = area(bx1, by1, bx2, by2);

        let ax_range = (ax1..=ax2);
        let ay_range = (ay1..=ay2);
        let bx_range = (bx1..=bx2);
        let by_range = (by1..=by2);

        match (
            ax_range.contains(&bx1),
            ax_range.contains(&bx2),
            ay_range.contains(&by1),
            ay_range.contains(&by2),
            bx_range.contains(&ax1),
            bx_range.contains(&ax2),
            by_range.contains(&ay1),
            by_range.contains(&ay2),
        ) {
            //outer
              (false, false, _, _, .., false, false, _, _)
            | (_, _, false, false, .., false, false) => return a_area + b_area,

            // a include b
            (true, true, true, true, ..) => return a_area,
            // b include a
            (.., true, true, true, true) => return b_area,

            // right intersection
            (true, false, true, true, ..) => return a_area + b_area - area(bx1, by1, ax2, by2),
            (true, false, false, true, ..) => return a_area + b_area - area(bx1, ay1, ax2, by2),
            (true, false, true, false, ..) => return a_area + b_area - area(bx1, by1, ax2, ay2),
            (true, false, false, false, ..) => return a_area + b_area - area(bx1, ay1, ax2, ay2),

            // left intersection
            (false, true, true, true, ..) => return a_area + b_area - area(ax1, by1, bx2, by2),
            (false, true, false, true, ..) => return a_area + b_area - area(ax1, ay1, bx2, by2),
            (false, true, true, false, ..) => return a_area + b_area - area(ax1, by1, bx2, ay2),
            (false, true, false, false, ..) => return a_area + b_area - area(ax1, ay1, bx2, ay2),

            // center intersection
            (true, true, false, false, ..) => return a_area + b_area - area(bx1, ay1, bx2, ay2),
            (true, true, false, true, ..) => return a_area + b_area - area(bx1, ay1, bx2, by2),
            (true, true, true, false, ..) => return a_area + b_area - area(bx1, by1, bx2, ay2),
            (false, false, true, true, ..) => return a_area + b_area - area(ax1, by1, ax2, by2),
            (false, false, false, true, ..) => return a_area + b_area - area(ax1, ay1, ax2, by2),
            (false, false, true, false, ..) => return a_area + b_area - area(ax1, by1, ax2, ay2),

            ignore => panic!("{:?}", ignore),
        }
    }
}

fn area(ax1: i32, ay1: i32, ax2: i32, ay2: i32) -> i32{
    println!("({},{}) ({},{})",ax1,ay1,ax2,ay2);
    ((ax2-ax1)*(ay2-ay1)).abs()
}