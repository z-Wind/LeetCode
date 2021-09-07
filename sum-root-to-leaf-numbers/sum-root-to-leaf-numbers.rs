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
impl Solution {
    pub fn sum_numbers(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        sum_numbers(root, 0)
    }
}

fn sum_numbers(root: Option<Rc<RefCell<TreeNode>>>, sum:i32) -> i32 {
    let val = root.as_ref().unwrap().borrow().val;
    let left = root.as_ref().unwrap().borrow_mut().left.take();
    let right = root.as_ref().unwrap().borrow_mut().right.take();
    match (left,right){
        (None,None) => sum*10+val,
        (Some(x),None) | (None,Some(x)) => {
            let s = sum*10+val;
            sum_numbers(Some(x), s)
        },
        (Some(l), Some(r)) => {
            let s = sum*10+val;
            sum_numbers(Some(l), s) + sum_numbers(Some(r), s)
        },
    }
}