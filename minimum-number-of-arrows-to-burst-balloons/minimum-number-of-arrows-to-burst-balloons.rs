impl Solution {
    pub fn find_min_arrow_shots(mut points: Vec<Vec<i32>>) -> i32 {
        points.sort_unstable();
        let mut count = 1;
        let mut cur = (points[0][0]..=points[0][1]);
        for p in &points[1..]{
            // println!("{:?} include? {:?}", cur, p);
            let start = p[0];
            let end = p[1];
            match (cur.contains(&start), cur.contains(&end)){
                (true, true) => {
                    cur = (start..=end);
                },
                (true, false) => {
                    cur = (start..=*cur.end());
                },
                (false, true) => {
                    cur = (*cur.start()..=end);
                },
                (false, false) => {
                    count += 1;
                    cur = (start..=end);
                }
            } 
        }
        count
    }
}