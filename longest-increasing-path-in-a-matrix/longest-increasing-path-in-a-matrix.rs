use std::cmp::max;
use std::cmp::Ordering;
impl Solution {
    pub fn longest_increasing_path(mut matrix: Vec<Vec<i32>>) -> i32 {
        let m = matrix.len();
        let n = matrix[0].len();
        let mut dp = vec![vec![0; n]; m];

        let mut ans = 0;
        for i in 0..m {
            for j in 0..n {
                longest_increasing_path(&mut ans, &mut dp, &matrix, i, j);
            }
        }
        // println!("[");
        // for i in 0..m {
        //     println!("{:?}", dp[i]);
        // }
        // println!("]");

        ans
    }
}

fn longest_increasing_path(
    ans: &mut i32,
    dp: &mut Vec<Vec<i32>>,
    matrix: &Vec<Vec<i32>>,
    i: usize,
    j: usize,
) -> i32 {
    let m = matrix.len();
    let n = matrix[0].len();

    // println!("{},{}",i,j);
    // println!("[");
    // for i in 0..m{
    //     println!("{:?}", dp[i]);
    // }
    // println!("]");

    if i >= m || j >= n {
        return 0;
    } else if dp[i][j] != 0 {
        return dp[i][j];
    }

    let mut update = |k: usize, l: usize| {
        if k >= m || l >= n {
            return 0;
        }
        // println!("{},{} => {},{}",i,j,k,l);
        match matrix[k][l].cmp(&matrix[i][j]) {
            Ordering::Greater => longest_increasing_path(ans, dp, matrix, k, l),
            Ordering::Less => 0,
            Ordering::Equal => 0,
        }
    };

    dp[i][j] = 1+
        // up
        update(i-1,j).max(
        // down
        update(i+1,j)).max(
        // left
        update(i,j-1)).max(
        // right
        update(i,j+1));

    *ans = (*ans).max(dp[i][j]);
    dp[i][j]
}
