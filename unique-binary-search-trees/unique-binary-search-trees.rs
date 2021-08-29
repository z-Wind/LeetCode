use std::rc::Rc;
use std::cell::RefCell;
use std::collections::HashMap;
impl Solution {
    pub fn num_trees(n: i32) -> i32 {
        let mut dp:HashMap<(i32,i32),i32> = HashMap::new();
        num_trees(1, n, &mut dp)
    }
}

fn num_trees(left:i32 ,right:i32, dp:&mut HashMap<(i32,i32),i32>) -> i32{
    if dp.contains_key(&(left,right)){
        return *dp.get(&(left,right)).unwrap();
    }
    //println!("{},{}", left, right);
    if left > right || left == right {
        return 1;
    }
    
    let mut count = 0;
    for root in (left..=right){
        count += (1 * num_trees(left,root-1,dp) * num_trees(root+1,right,dp));
    }
    
    dp.insert((left,right), count);
    count
}