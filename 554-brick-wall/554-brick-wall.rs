use std::collections::HashSet;

impl Solution {
    pub fn least_bricks(wall: Vec<Vec<i32>>) -> i32 {
        let n = wall.len();
        let mut lines = HashSet::new();
        let mut edges: Vec<HashSet<i32>> = vec![HashSet::new(); n];
        for i in 0..n {
            let mut edge = 0;
            for j in 0..wall[i].len() - 1 {
                edge += wall[i][j];
                edges[i].insert(edge);
                lines.insert(edge);
            }
        }
        // println!("{:?}", edges);

        let mut ans = n as i32;
        let width: i32 = wall[0].iter().sum();
        for pos in lines {
            let mut count = n as i32;
            for i in 0..n {
                if edges[i].contains(&pos) {
                    count -= 1;
                }
            }
            ans = ans.min(count);
        }
        
        ans
    }
}
