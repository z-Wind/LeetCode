use std::collections::HashMap;
impl Solution {
    pub fn number_of_boomerangs(points: Vec<Vec<i32>>) -> i32 {
        let n = points.len();
        let mut ans = 0;
        let mut map: Vec<HashMap<i64, i32>> = vec![HashMap::new(); n];
        for i in 0..n {
            for j in i + 1..n {
                let x: i64 = (points[i][0] - points[j][0]) as i64;
                let y: i64 = (points[i][1] - points[j][1]) as i64;
                let dis: i64 = x * x + y * y;
                *map[i].entry(dis).or_insert(0) += 1;
                *map[j].entry(dis).or_insert(0) += 1;
            }
            ans += map[i].values().map(|&count| count * (count - 1)).sum::<i32>();
        }
        // println!("{:?}", map);

        ans
    }
}
