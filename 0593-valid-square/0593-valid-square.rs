impl Solution {
    pub fn valid_square(p1: Vec<i32>, p2: Vec<i32>, p3: Vec<i32>, p4: Vec<i32>) -> bool {
        let mut points = [
            (&p2, len(&p1, &p2)),
            (&p3, len(&p1, &p3)),
            (&p4, len(&p1, &p4)),
        ];
        points.sort_unstable_by_key(|x| x.1);

        points[0].1 != 0
            && points[0].1 == points[1].1
            && points[1].1 != points[2].1
            && inner_product(&p1, points[0].0, points[1].0) == 0
            && inner_product(points[2].0, points[0].0, points[1].0) == 0
    }
}

fn len(p1: &[i32], p2: &[i32]) -> i32 {
    (p1[0] - p2[0]) * (p1[0] - p2[0]) + (p1[1] - p2[1]) * (p1[1] - p2[1])
}

// https://zh.wikipedia.org/zh-tw/%E7%82%B9%E7%A7%AF
fn inner_product(p1: &[i32], p2: &[i32], p3: &[i32]) -> i32 {
    // V1: (p1, p2)
    // V2: (p1, p3)
    let v1 = (p2[0] - p1[0], p2[1] - p1[1]);
    let v2 = (p3[0] - p1[0], p3[1] - p1[1]);

    v1.0 * v2.0 + v1.1 * v2.1
}
