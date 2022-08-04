impl Solution {
    pub fn find_min_difference(time_points: Vec<String>) -> i32 {
        let mut time_points: Vec<i32> = time_points.into_iter().map(time_to_num).collect();
        time_points.sort_unstable();
        
        let mut ans = i32::MAX;
        for w in time_points.windows(2) {
            ans = ans.min(w[1] - w[0]);
        }

        ans.min(time_points[0] + 60 * 24 - time_points.last().unwrap())
    }
}

fn time_to_num(time: String) -> i32 {
    time.split(":").enumerate().fold(0, |acc, (i, t)| {
        acc + t.parse::<i32>().unwrap() * (-59 * i as i32 + 60)
    })
}
