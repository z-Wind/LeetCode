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
    if root.is_none() {
        return false;
    }

    let left = &root.as_ref().unwrap().borrow().left;
    let right = &root.as_ref().unwrap().borrow().right;

    is_same(root, sub_root) || is_subtree(left, sub_root) || is_subtree(right, sub_root)
}

fn is_same(root: &Option<Rc<RefCell<TreeNode>>>, sub_root: &Option<Rc<RefCell<TreeNode>>>) -> bool {
    match (root, sub_root) {
        (None, None) => true,
        (Some(node), Some(sub_node)) => {
            let left = &node.borrow().left;
            let right = &node.borrow().right;
            let sub_left = &sub_node.borrow().left;
            let sub_right = &sub_node.borrow().right;
            node.borrow().val == sub_node.borrow().val
                && is_same(left, sub_left)
                && is_same(right, sub_right)
        }
        _ => false,
    }
}
