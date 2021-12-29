impl Solution {
    pub fn find_min_arrow_shots(mut points: Vec<Vec<i32>>) -> i32 {
        points.sort_unstable_by_key(|v| v[1]);
        // println!("{:?}", points);
        
        let mut count = 1;
        let mut end = points[0][1];
        for i in 1..points.len(){
            if end < points[i][0]{
                end = points[i][1];
                count += 1;
            } 
        }
        count
    }
}