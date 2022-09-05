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
    pub fn find_tilt(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let (_, tilt) = find_tilt(root);
        tilt
    }
}

fn find_tilt(root: Option<Rc<RefCell<TreeNode>>>) -> (i32, i32) {
    if root.is_none() {
        return (0, 0);
    }

    let left = root.as_ref().unwrap().borrow().left.clone();
    let (sum_left, tilt_left) = find_tilt(left);

    let right = root.as_ref().unwrap().borrow().right.clone();
    let (sum_right, tilt_right) = find_tilt(right);

    let val = root.as_ref().unwrap().borrow().val;
    return (
        val + sum_left + sum_right,
        tilt_left + tilt_right + (sum_left - sum_right).abs(),
    );
}
