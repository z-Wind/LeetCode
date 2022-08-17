// https://leetcode.com/problems/remove-boxes/discuss/101310/Java-top-down-and-bottom-up-DP-solutions

// T(i, j, k) as the maximum points one can get by removing boxes of the subarray boxes[i, j] (both inclusive), and with k boxes attached to its left of the same color as boxes[i]
// max(T(i + 1, m - 1, 0) + T(m, j, k + 1)) where i < m <= j && boxes[i] == boxes[m]
// The termination conditions now will be:
// a. T(i, i - 1, k) = 0: no boxes so no points, and this is true for any k (you can interpret it as nowhere to attach the boxes).
// b. T(i, i, k) = (k + 1) * (k + 1): only one box left in the subarray but we've already got k boxes of the same color attached to its left, so the total number of boxes of the same color is (k + 1) and the maximum point is (k + 1) * (k + 1).

impl Solution {
    pub fn remove_boxes(boxes: Vec<i32>) -> i32 {
        let n = boxes.len();
        let mut dp = vec![vec![vec![0; n]; n]; n];
        remove_boxes(&mut dp, &boxes, 0, n - 1, 0)
    }
}

fn remove_boxes(
    dp: &mut Vec<Vec<Vec<i32>>>,
    boxes: &Vec<i32>,
    i: usize,
    j: usize,
    k: usize,
) -> i32 {
    if i > j {
        return 0;
    } else if i == j {
        return ((k + 1) * (k + 1)) as i32;
    } else if dp[i][j][k] > 0 {
        return dp[i][j][k];
    }
    
    let results = {
        let mut points = 0;
        let mut i = i;
        let mut k = k;
        
        // combine
        while i < j && boxes[i] == boxes[i + 1] {
            i += 1;
            k += 1;
        }

        let mut points = remove_boxes(dp, boxes, i, i, k) + remove_boxes(dp, boxes, i + 1, j, 0);
        for l in i + 1..=j {
            if boxes[l] == boxes[i] {
                points = points.max(
                    remove_boxes(dp, boxes, i + 1, l - 1, 0) + remove_boxes(dp, boxes, l, j, k + 1),
                );
            }
        }
        points
    };

    dp[i][j][k] = results;
    return results;
}
