use std::collections::BTreeMap;

impl Solution {
    pub fn find_right_interval(intervals: Vec<Vec<i32>>) -> Vec<i32> {
        let n = intervals.len();
        let mut map: BTreeMap<i32, usize> = BTreeMap::new();
        for i in 0..n {
            map.insert(intervals[i][0], i);
        }
        // println!("{:?}", map);

        let mut ans = vec![-1; n];
        for i in 0..n {
            if let Some((_, &idx)) = map.range(intervals[i][1]..).next() {
                ans[i] = idx as i32;
            }
        }
        ans
    }
}
