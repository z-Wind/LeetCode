// https://leetcode.com/problems/student-attendance-record-ii/discuss/415467/Python-O(log-n)-using-NumPy
// finite state machine:
//    S(n)      =      M        x    S(n-1)
// /        \   /             \   /          \
// | S(n,0) |   | 1 1 1 0 0 0 |   | S(n-1,0) |
// | S(n,1) |   | 1 0 0 0 0 0 |   | S(n-1,1) |
// | S(n,2) | = | 0 1 0 0 0 0 | x | S(n-1,2) |
// | S(n,3) |   | 1 1 1 1 1 1 |   | S(n-1,3) |
// | S(n,4) |   | 0 0 0 1 0 0 |   | S(n-1,4) |
// | S(n,5) |   | 0 0 0 0 1 0 |   | S(n-1,5) |
// \        /   \             /   \          /
//
// S(0) = [1 0 0 0 0 0]^T

fn matrix_mul(A: &Vec<Vec<i64>>, B: &Vec<Vec<i64>>) -> Vec<Vec<i64>> {
    let m = A.len();
    let n = A[0].len();
    let p = B[0].len();

    let mut result = vec![vec![0; p]; m];
    for i in 0..m {
        for j in 0..p {
            result[i][j] = (0..n).fold(0, |acc, k| (acc + A[i][k] * B[k][j]) % 1_000_000_007);
        }
    }

    result
}

fn pow(mut M: Vec<Vec<i64>>, mut n: i32) -> Vec<Vec<i64>> {
    let m = M.len();
    let mut res = vec![vec![0; m]; m];
    // identity matrix
    for i in 0..m {
        res[i][i] = 1;
    }
    
    while n > 0 {
        if n % 2 == 1 {
            res = matrix_mul(&res, &M);
        }
        M = matrix_mul(&M, &M);
        n /= 2;
    }
    return res;
}

impl Solution {
    pub fn check_record(n: i32) -> i32 {
        let A = vec![
            vec![1, 1, 1, 0, 0, 0],
            vec![1, 0, 0, 0, 0, 0],
            vec![0, 1, 0, 0, 0, 0],
            vec![1, 1, 1, 1, 1, 1],
            vec![0, 0, 0, 1, 0, 0],
            vec![0, 0, 0, 0, 1, 0],
        ];
        
        // sum(S(n)) => S(n+1, 3) ∵ M[3,:] = 1 1 1 1 1 1
        // S(n+1) = M^(n+1) x S(0)
        // S(n+1, 3) => M^(n+1)[3,0] ∵ S(0) = [1 0 0 0 0 0]^T
        pow(A, n + 1)[3][0] as i32
    }
}
