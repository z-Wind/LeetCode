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
    pub fn diameter_of_binary_tree(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut diameter = 0;
        length_node(root.as_ref(), &mut diameter);
        diameter
    }
}

fn length_node(root: Option<&Rc<RefCell<TreeNode>>>, diameter: &mut i32) -> (i32, i32) {
    let root = root.unwrap().borrow();
    let left = root.left.as_ref();
    let right = root.right.as_ref();

    let left_len = if left.is_some() {
        let (l, r) = length_node(left, diameter);
        l.max(r) + 1
    } else {
        0
    };
    let right_len = if right.is_some() {
        let (l, r) = length_node(right, diameter);
        l.max(r) + 1
    } else {
        0
    };

    *diameter = (*diameter).max(left_len + right_len);
    (left_len, right_len)
}
