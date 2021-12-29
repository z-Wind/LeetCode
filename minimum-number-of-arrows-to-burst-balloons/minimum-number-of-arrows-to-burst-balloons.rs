impl Solution {
    pub fn find_min_arrow_shots(mut points: Vec<Vec<i32>>) -> i32 {
        points.sort_unstable();
        // println!("{:?}", points);
        
        let mut count = 1;
        let mut end = points[0][1];
        for p in &points[1..]{
            if end >= p[0]{
                end = end.min(p[1]);
            } else {
                end = p[1];
                count += 1;
            } 
        }
        count
    }
}