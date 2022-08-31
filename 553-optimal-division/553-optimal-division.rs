impl Solution {
    pub fn optimal_division(nums: Vec<i32>) -> String {
        let n = nums.len();
        let nums:Vec<f32> = nums.into_iter().map(|x| x as f32).collect();

        let mut dp = vec![vec![((-1.0, String::new()), (-1.0, String::new())); n]; n];
        for i in 0..n {
            dp[i][i] = ((nums[i], nums[i].to_string()), (nums[i], nums[i].to_string()));
        }
        let ans = optimal_result(&mut dp, &nums, 0, n - 1);
        ans.1 .1
    }
}

fn optimal_result(
    dp: &mut Vec<Vec<((f32, String), (f32, String))>>,
    nums: &Vec<f32>,
    start: usize,
    end: usize,
) -> ((f32, String), (f32, String)) {
    if dp[start][end].0.0 >= 0.0 {
        return dp[start][end].clone();
    }

    let mut minimal = (f32::INFINITY, String::new());
    let mut maximal = (f32::NEG_INFINITY, String::new());

    for i in start..end {
        let a = optimal_result(dp, nums, start, i);
        let b = optimal_result(dp, nums, i + 1, end);

        let rst_min = if end != i + 1 {
            (a.0 .0 / b.1 .0, format!("{}/({})", a.0 .1, b.1 .1))
        } else {
            (a.0 .0 / b.1 .0, format!("{}/{}", a.0 .1, b.1 .1))
        };
        if rst_min.0 < minimal.0 {
            minimal = rst_min;
        }

        let rst_max = if end != i + 1 {
            (a.1 .0 / b.0 .0, format!("{}/({})", a.1 .1, b.0 .1))
        } else {
            (a.1 .0 / b.0 .0, format!("{}/{}", a.1 .1, b.0 .1))
        };
        if rst_max.0 > maximal.0 {
            maximal = rst_max;
        }
    }
    // println!("{:?} => {:?}, {:?}", &nums[start..=end], minimal, maximal);
    
    dp[start][end] = (minimal, maximal);
    
    return dp[start][end].clone();
}
