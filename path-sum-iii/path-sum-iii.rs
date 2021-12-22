// Definition for a binary tree node.
// #[derive(Debug, PartialEq, Eq)]
// pub struct TreeNode {
//   pub val: i32,
//   pub left: Option<Rc<RefCell<TreeNode>>>,
//   pub right: Option<Rc<RefCell<TreeNode>>>,
// }
// 
// impl TreeNode {
//   #[inline]
//   pub fn new(val: i32) -> Self {
//     TreeNode {
//       val,
//       left: None,
//       right: None
//     }
//   }
// }

use std::collections::HashMap;

use std::rc::Rc;
use std::cell::RefCell;
impl Solution {
    pub fn path_sum(root: Option<Rc<RefCell<TreeNode>>>, target_sum: i32) -> i32 {
        let (.., ans) = path_sum(root.as_ref(), target_sum as i64);
        ans
    }
}

fn path_sum(root: Option<&Rc<RefCell<TreeNode>>>, target_sum: i64) -> (Vec<i64>, i32) {
    if root.is_none(){
        return (Vec::new(), 0);
    }
    
    let mut count = 0;
    let val = root.as_ref().unwrap().borrow().val as i64;
    if val == target_sum{
        count += 1;
    }
    
    let mut left = path_sum(root.as_ref().unwrap().borrow().left.as_ref(), target_sum);
    for sum in left.0.iter_mut(){
        *sum += val;
        if *sum == target_sum{
            count += 1;
        }
    }
    count += left.1;
    
    let mut right = path_sum(root.as_ref().unwrap().borrow().right.as_ref(), target_sum);
    for sum in right.0.iter_mut(){
        *sum += val;
        if *sum == target_sum{
            count += 1;
        }
    }
    count += right.1;
    
    left.0.append(&mut right.0);
    left.0.push(val);
    
    (left.0, count)
    
}