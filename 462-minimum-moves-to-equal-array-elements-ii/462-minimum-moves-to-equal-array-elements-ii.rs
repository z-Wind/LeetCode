use std::collections::BTreeMap;
impl Solution {
    pub fn min_moves2(nums: Vec<i32>) -> i32 {
        let mut map = BTreeMap::new();
        for num in nums {
            *map.entry(num as i64).or_insert(0) += 1;
        }

        let mut mov_min = i64::MAX;
        'outer: for &target in map.keys() {
            let mut mov = 0;
            for (num, count) in map.iter() {
                mov += (target - *num).abs() * *count;
                if mov >= mov_min{
                    break 'outer;
                }
            }
            
            mov_min = mov;
        }
        mov_min as i32
    }
}
