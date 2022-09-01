use std::collections::HashMap;

impl Solution {
    pub fn least_bricks(wall: Vec<Vec<i32>>) -> i32 {
        let n = wall.len();
        let mut lines = HashMap::new();
        for row in wall {
            let mut edge = 0;
            for j in 0..row.len() - 1 {
                edge += row[j];
                lines
                    .entry(edge)
                    .and_modify(|counter| *counter -= 1)
                    .or_insert(n as i32 - 1);
            }
        }

        if lines.is_empty() {
            return n as i32;
        }
        lines.into_values().min().unwrap()
    }
}
