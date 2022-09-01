use std::collections::HashMap;

impl Solution {
    pub fn least_bricks(wall: Vec<Vec<i32>>) -> i32 {
        let n = wall.len();
        let width: i32 = wall[0].iter().sum();
        let mut lines = HashMap::new();
        for i in 0..n {
            let mut edge = 0;
            for j in 0..wall[i].len() - 1 {
                edge += wall[i][j];
                lines.entry(edge).and_modify(|counter| *counter -= 1).or_insert(n as i32 - 1);
            }
        }
        
        if lines.is_empty() {
            return n as i32;
        }
        lines.into_values().min().unwrap()
    }
}