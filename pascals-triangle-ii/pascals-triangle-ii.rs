// C(n,k) => (x+1)^n
// C(n,k) = C(n, k-1) * (n-(k-1)) / k
use std::iter;
impl Solution {
    pub fn get_row(row_index: i32) -> Vec<i32> {
        // for overflow
        let row_index = row_index as usize;
        
        iter::once(1)
            .chain((1..row_index+1).scan(1, |prev, i| {
                *prev = (*prev as usize * (row_index - (i-1)) / i) as i32;
                Some(*prev)
            }))
            .collect::<Vec<i32>>()
    }
}