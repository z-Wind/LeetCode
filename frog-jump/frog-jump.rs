use std::collections::HashMap;
impl Solution {
    pub fn can_cross(stones: Vec<i32>) -> bool {
        let mut dp:HashMap<(usize,i32),bool> = HashMap::new();
        if stones[1] != stones[0]+1{
            return false;
        }
        can_cross(&mut dp, &stones, 1, 1)
    }
}

fn can_cross(dp:&mut HashMap<(usize,i32),bool>,stones: &Vec<i32>, start:usize, step:i32) -> bool {
    if start == stones.len()-1{
        return true;
    }
    if dp.contains_key(&(start,step)){
        return *dp.get(&(start,step)).unwrap();
    }
    
    // println!("{} step:{}",stones[start],step);
    let mut ans = false;
    for i in start+1..stones.len(){
        let gap = stones[i] - stones[start];
        if  gap == step - 1 && can_cross(dp, &stones, i, step-1){
            ans = true;
            break;
        } else if gap == step && can_cross(dp, &stones, i, step){
            ans = true;
            break;
        } else if gap == step + 1 && can_cross(dp, &stones, i, step+1){
            ans = true;
            break;
        } else if gap > step + 1 {
            break;
        }
    }
    
    dp.insert((start,step), ans);
    ans
}