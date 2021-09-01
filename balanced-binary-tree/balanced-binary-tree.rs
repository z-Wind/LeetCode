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
use std::rc::Rc;
use std::cell::RefCell;
use std::collections::VecDeque;
impl Solution {
    pub fn is_balanced(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        let ans = is_balanced(root);
        ans.1
    }
}

fn is_balanced(root: Option<Rc<RefCell<TreeNode>>>) -> (i32, bool){
    if root.is_none(){
        return (0,true);
    }
    let left = root.as_ref().unwrap().borrow().left.clone();
    let right = root.as_ref().unwrap().borrow().right.clone();
    
    let (left_layer, left_balance) = is_balanced(left);
    if !left_balance {return (-1,false)}
    let (right_layer, right_balance) = is_balanced(right);
    if !right_balance {return (-1,false)}
    
    (left_layer.max(right_layer)+1, (left_layer-right_layer).abs()<=1 && left_balance && right_balance)
}
