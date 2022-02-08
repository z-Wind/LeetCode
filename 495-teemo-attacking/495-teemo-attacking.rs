impl Solution {
    pub fn find_poisoned_duration(time_series: Vec<i32>, duration: i32) -> i32 {
        let mut i = 0;
        let mut poisoned_time = 0;
        let mut t = 0;
        while i < time_series.len() {
            if t <= time_series[i] {
                poisoned_time += duration;
            } else {
                poisoned_time += (time_series[i] - time_series[i - 1]);
            }
            t = time_series[i] + duration;
            i += 1;
        }
        
        poisoned_time
    }
}