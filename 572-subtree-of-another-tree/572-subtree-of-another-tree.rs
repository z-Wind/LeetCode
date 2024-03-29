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

// https://leetcode.com/problems/subtree-of-another-tree/discuss/1676378/Rust-4ms-or-2.3

use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    pub fn is_subtree(
        root: Option<Rc<RefCell<TreeNode>>>,
        sub_root: Option<Rc<RefCell<TreeNode>>>,
    ) -> bool {
        is_subtree(&root, &sub_root)
    }
}

fn is_subtree(
    root: &Option<Rc<RefCell<TreeNode>>>,
    sub_root: &Option<Rc<RefCell<TreeNode>>>,
) -> bool {
    if root == sub_root {
        return true;
    }
    if let Some(node) = root {
        let node = node.borrow();
        is_subtree(&node.left, sub_root) || is_subtree(&node.right, sub_root)
    } else {
        return false;
    }
}
