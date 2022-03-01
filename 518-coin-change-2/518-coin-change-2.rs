// https://leetcode.com/problems/coin-change-2/discuss/99212/Knapsack-problem-Java-solution-with-thinking-process-O(nm)-Time-and-O(m)-Space

// ith coins & amount j
// dp[i][j] = dp[i-1][j] + dp[i][j-coins[i-1]]

//                     dp[i-1][j]
// dp[i][j-coins[i-1]  dp[i][j]

// dp[j-coins[i-1] dp[j](update)

impl Solution {
    pub fn change(amount: i32, coins: Vec<i32>) -> i32 {
        // change(amount, coins)
        change_1D(amount, coins)
    }
}

fn change(amount: i32, coins: Vec<i32>) -> i32 {
    let amount = amount as usize;

    let mut dp = vec![vec![0; amount + 1]; coins.len() + 1];
    dp[0][0] = 1;
    for i in 1..=coins.len() {
        dp[i][0] = 1;
        for j in 1..=amount {
            dp[i][j] = dp[i - 1][j];
            if j >= coins[i - 1] as usize {
                dp[i][j] += dp[i][j - coins[i - 1] as usize]
            }
        }
    }
    // println!("{:?}", dp);

    dp[coins.len()][amount]
}

fn change_1D(amount: i32, coins: Vec<i32>) -> i32 {
    let amount = amount as usize;

    let mut dp = vec![0; amount + 1];
    dp[0] = 1;
    for coin in coins {
        let coin = coin as usize;
        for j in coin..=amount {
            dp[j] += dp[j - coin]
        }
    }
    // println!("{:?}", dp);

    dp[amount]
}
