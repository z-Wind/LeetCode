use std::collections::BTreeMap;

impl Solution {
    pub fn erase_overlap_intervals(intervals: Vec<Vec<i32>>) -> i32 {
        let n = intervals.len();
        let mut map:BTreeMap<i32, i32> = BTreeMap::new();
        for interval in intervals{
            match map.get_mut(&interval[0]){
                None => {
                    map.insert(interval[0], interval[1]);
                },
                Some(x) => {
                    if *x > interval[1]{
                        *x = interval[1];
                    }
                },
            }
        }        
        // println!("{:?}", map);
        
        let mut count = 1;
        let mut min_end = i32::MAX;
        for (&start, &end) in map.iter(){
            if start < min_end && end < min_end{
                min_end = end;
            } else if start >= min_end {
                count += 1;
                min_end = end;
            }
        }
        // println!("count:{}", count);
        n as i32 - count
    }
}