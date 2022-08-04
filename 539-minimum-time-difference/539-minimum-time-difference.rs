// https://leetcode.com/problems/minimum-time-difference/discuss/100640/Verbose-Java-Solution-Bucket/208661

impl Solution {
    pub fn find_min_difference(time_points: Vec<String>) -> i32 {
        let mut mark = vec![false; 24 * 60];
        let mut start = usize::MAX;
        let mut end = usize::MIN;
        for num in time_points.into_iter().map(time_to_num) {
            let num = num as usize;
            mark[num] = match mark[num] {
                true => return 0,
                false => {
                    start = start.min(num);
                    end = end.max(num);
                    true
                }
            }
        }

        let mut ans = start + 60 * 24 - end;
        let mut prev = start;
        for i in start + 1..=end {
            if mark[i] {
                ans = ans.min(i - prev);
                prev = i;
            }
        }

        ans as i32
    }
}

fn time_to_num(time: String) -> i32 {
    time.split(":").enumerate().fold(0, |acc, (i, t)| {
        acc + t.parse::<i32>().unwrap() * (-59 * i as i32 + 60)
    })
}
