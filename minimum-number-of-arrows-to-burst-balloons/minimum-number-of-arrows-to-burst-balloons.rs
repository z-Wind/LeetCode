impl Solution {
    pub fn find_min_arrow_shots(mut points: Vec<Vec<i32>>) -> i32 {
        points.sort_unstable_by_key(|v| v[1]);
        // println!("{:?}", points);
        
        let mut count = 1;
        let mut arrow = points[0][1];
        for p in &points[1..]{
            if arrow < p[0]{
                arrow = p[1];
                count += 1;
            } 
        }
        count
    }
}