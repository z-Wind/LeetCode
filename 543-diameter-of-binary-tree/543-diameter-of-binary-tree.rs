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

// https://leetcode.com/problems/diameter-of-binary-tree/discuss/101132/Java-Solution-MaxDepth

use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    pub fn diameter_of_binary_tree(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut diameter = 0;
        max_depth(root.as_ref(), &mut diameter);
        diameter
    }
}

fn max_depth(root: Option<&Rc<RefCell<TreeNode>>>, diameter: &mut i32) -> i32 {
    if root.is_none() {
        return 0;
    }
    let root = root.unwrap().borrow();
    let left = root.left.as_ref();
    let right = root.right.as_ref();

    let left_len = max_depth(left, diameter);
    let right_len = max_depth(right, diameter);

    *diameter = (*diameter).max(left_len + right_len);
    left_len.max(right_len) + 1
}
