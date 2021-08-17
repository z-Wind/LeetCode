impl Solution {
    pub fn insert(mut intervals: Vec<Vec<i32>>, new_interval: Vec<i32>) -> Vec<Vec<i32>> {
        if intervals.len() == 0{
            return vec![new_interval];
        }
        if new_interval[0] > intervals[intervals.len()-1][1]{
            intervals.push(new_interval);
            return intervals;
        }
        
        let mut split_index = 0;
        let mut insert_index = 1;
        for (i,interval) in intervals.iter().enumerate(){
            if i == 0 && interval[0] > new_interval[0]{
                split_index = 0;
                insert_index = 0;
                break;
            } else if interval[0] > new_interval[0]{
                split_index = i -1;
                break;
            } else if i == intervals.len() -1 && interval[0] <= new_interval[0] && new_interval[0] <= interval[1]{
                split_index = i;
                break;
            }
        }
        //println!("split_index: {}", split_index);
        let mut right = intervals.split_off(split_index);
        right.insert(insert_index, new_interval);
        
        right = merge(right);
        intervals.append(&mut right);
        intervals
    }
}

fn merge(mut intervals: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
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