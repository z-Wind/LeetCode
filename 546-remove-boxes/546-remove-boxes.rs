// https://leetcode.com/problems/remove-boxes/discuss/101310/Java-top-down-and-bottom-up-DP-solutions

// T(i, j, k) as the maximum points one can get by removing boxes of the subarray boxes[i, j] (both inclusive), and with k boxes attached to its left of the same color as boxes[i]
// max(T(i + 1, m - 1, 0) + T(m, j, k + 1)) where i < m <= j && boxes[i] == boxes[m]
// The termination conditions now will be:
// a. T(i, i - 1, k) = 0: no boxes so no points, and this is true for any k (you can interpret it as nowhere to attach the boxes).
// b. T(i, i, k) = (k + 1) * (k + 1): only one box left in the subarray but we've already got k boxes of the same color attached to its left, so the total number of boxes of the same color is (k + 1) and the maximum point is (k + 1) * (k + 1).

use std::collections::HashMap;

impl Solution {
    pub fn remove_boxes(boxes: Vec<i32>) -> i32 {
        let mut dp = HashMap::new();
        remove_boxes(&mut dp, &boxes, 0, boxes.len() - 1, 0)
    }
}

fn remove_boxes(
    dp: &mut HashMap<(usize, usize, i32), i32>,
    boxes: &Vec<i32>,
    mut i: usize,
    j: usize,
    mut k: i32,
) -> i32 {
    let key = (i, j, k);
    if let Some(&p) = dp.get(&key) {
        return p;
    }
    if i > j {
        return 0;
    } else if i == j {
        return (k + 1) * (k + 1);
    }
    
    // combine
    while i < j {
        if boxes[i] == boxes[i+1] {
            i += 1;
            k += 1;
        } else {
            break;
        }
    }
    
    let mut points = remove_boxes(dp, boxes, i, i, k) + remove_boxes(dp, boxes, i + 1, j, 0);
    for l in i + 1..=j {
        if boxes[l] == boxes[i] {
            points = points.max(
                remove_boxes(dp, boxes, i + 1, l - 1, 0) + remove_boxes(dp, boxes, l, j, k + 1),
            );
        }
    }

    dp.insert(key, points);
    return points;
}
