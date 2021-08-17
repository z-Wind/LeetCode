impl Solution {
    pub fn merge(mut intervals: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        if intervals.len() == 1{
            return intervals;
        }
        
        intervals.sort_by(|a, b| a[0].cmp(&b[0]));
        //println!("intervals: {:?}",intervals);
        
        let mut ans:Vec<Vec<i32>> = Vec::new();
        'outer: for interval in intervals.into_iter(){
            //println!("interval: {:?} ans: {:?}",interval, ans);
            for rang in ans.iter_mut(){
                if rang[0] <= interval[0] && interval[1] <= rang[1]{
                    continue 'outer;
                } else if interval[0] < rang[0] && rang[1] < interval[1]{
                    rang[0] = interval[0];
                    rang[1] = interval[1];
                    continue 'outer;
                } else if rang[0] <= interval[0] && interval[0] <= rang[1]{
                    rang[1] = rang[1].max(interval[1]);
                    continue 'outer;
                } else if rang[0] <= interval[1] && interval[1] <= rang[1]{
                    rang[0] = rang[0].min(interval[0]);
                    continue 'outer;
                }
            }  
            ans.push(interval);
        }
        ans
    }
}