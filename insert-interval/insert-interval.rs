impl Solution {
    pub fn insert(intervals: Vec<Vec<i32>>, new_interval: Vec<i32>) -> Vec<Vec<i32>> {
        let mut left:Vec<Vec<i32>> = vec![];
        let mut right:Vec<Vec<i32>> = vec![];
        
        let mut new = new_interval.clone();
        for interval in intervals{
            if interval[0] > new_interval[1]{
                right.push(interval);
            } else if interval[1] < new_interval[0]{
                left.push(interval);
            } else {
                new[0] = new[0].min(interval[0]);
                new[1] = new[1].max(interval[1]);
            }
        }
        left.push(new);
        left.append(&mut right);
        left
    }
}
