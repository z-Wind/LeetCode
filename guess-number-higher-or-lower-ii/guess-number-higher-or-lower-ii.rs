// https://leetcode.com/problems/guess-number-higher-or-lower-ii/discuss/84764/Simple-DP-solution-with-explanation~~
// result_when_pick_x = x + max{DP([i~x-1]), DP([x+1, j])}
// --> // the max means whenever you choose a number, the feedback is always bad and therefore leads you to a worse branch.
// then we get DP([i~j]) = min{xi, ... ,xj}
// --> // this min makes sure that you are minimizing your cost.

impl Solution {
    pub fn get_money_amount(n: i32) -> i32 {
        let n = n as usize;
        let mut dp: Vec<Vec<i32>> = vec![vec![0; n+1]; n+1];
        get_money_amount(&mut dp, 1, n)
    }
}

fn get_money_amount(dp: &mut Vec<Vec<i32>>, start: usize, end: usize) -> i32 {
    if start >= end {
        return 0;
    }
    if dp[start][end] != 0 {
        return dp[start][end];
    }

    let mut res: i32 = i32::MAX;
    let mut tt = 0;
    for x in start..=end {
        let ans = x as i32 + get_money_amount(dp, start, x - 1).max(get_money_amount(dp, x + 1, end));
        if res > ans{
            tt = x;
            res = ans;
        }
    }
    // println!("{},{} => {}:{}",start,end,tt,res);
    dp[start][end] = res;
    res
}
