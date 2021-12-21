use std::collections::BTreeMap;

impl Solution {
    pub fn erase_overlap_intervals(intervals: Vec<Vec<i32>>) -> i32 {
        let n = intervals.len();
        let mut map:BTreeMap<i32, i32> = BTreeMap::new();
        for interval in intervals{
            match map.get_mut(&interval[1]){
                None => {
                    map.insert(interval[1], interval[0]);
                },
                Some(x) => {
                    if *x < interval[0]{
                        *x = interval[0];
                    }
                },
            }
        }        
        // println!("{:?}", map);
        
        let mut count = 1;
        let mut min_end = *map.keys().next().unwrap();
        for (&end, &start) in map.iter(){
            if start >= min_end {
                count += 1;
                min_end = end;
            }
        }
        // println!("count:{}", count);
        n as i32 - count
    }
}