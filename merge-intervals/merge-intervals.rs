impl Solution {
    pub fn merge(mut intervals: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        if intervals.len() == 1{
            return intervals;
        }
        
        intervals.sort_by(|a, b| a[0].cmp(&b[0]));
        //println!("intervals: {:?}",intervals);
        
        let mut ans:Vec<Vec<i32>> = Vec::new();
        for interval in intervals.into_iter(){
            if ans.is_empty() || ans[ans.len()-1][1] < interval[0]{
                ans.push(interval);    
            } else {
                let n = ans.len();
                ans[n-1][1] = ans[n-1][1].max(interval[1]);
            }
        }
        ans
    }
}