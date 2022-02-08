impl Solution {
    pub fn find_poisoned_duration(time_series: Vec<i32>, duration: i32) -> i32 {
        let mut poisoned_time = duration; // last
        for i in 0..time_series.len() - 1 {
            poisoned_time += duration.min(time_series[i + 1] - time_series[i]);
        }

        poisoned_time
    }
}
