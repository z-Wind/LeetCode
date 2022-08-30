// f(n, L, A) = P: f(n-1, 0, A) +
//              L: f(n-1, L+1, A) +
//              A: f(n-1, 0, A+1)

impl Solution {
    pub fn check_record(n: i32) -> i32 {
        let n = n as usize;
        let mut dp = vec![vec![vec![0; 2]; 3]; n + 1];
        nums_possible_records(&mut dp, n, 0, 0)
    }
}

fn nums_possible_records(dp: &mut Vec<Vec<Vec<i32>>>, n: usize, L: usize, A: usize) -> i32 {
    if n == 0 || L >= 3 || A >= 2 {
        return 0;
    } else if dp[n][L][A] > 0 {
        return dp[n][L][A];
    }

    dp[n][L][A] = if n == 1 {
        let Lc = if L < 2 { 1 } else { 0 };
        let Ac = if A < 1 { 1 } else { 0 };
        1 + Lc + Ac
    } else {
        (((nums_possible_records(dp, n-1, 0, A) % 1_000_000_007    // P
      + nums_possible_records(dp, n-1, L+1, A) % 1_000_000_007) % 1_000_000_007)  // L
      + nums_possible_records(dp, n-1, 0, A+1)) % 1_000_000_007  // A
    };

    dp[n][L][A]
}
