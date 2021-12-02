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
use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    pub fn sum_of_left_leaves(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        sum_of_left_leaves(root, false)
    }
}

fn sum_of_left_leaves(root: Option<Rc<RefCell<TreeNode>>>, is_left_leaf: bool) -> i32 {
    let left = root.as_ref().unwrap().borrow().left.clone();
    let right = root.as_ref().unwrap().borrow().right.clone();
    match (left, right) {
        (None, None) => {
            if is_left_leaf {
                return root.as_ref().unwrap().borrow().val;
            } else {
                return 0;
            }
        }
        (left, None) => sum_of_left_leaves(left, true),
        (None, right) => sum_of_left_leaves(right, false),
        (left, right) => sum_of_left_leaves(left, true) + sum_of_left_leaves(right, false),
    }
}
