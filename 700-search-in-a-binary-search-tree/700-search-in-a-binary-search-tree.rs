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
use std::cmp::Ordering;

impl Solution {
    pub fn search_bst(root: Option<Rc<RefCell<TreeNode>>>, val: i32) -> Option<Rc<RefCell<TreeNode>>> {
        if root.is_none() {
            return None;
        }
        
        let cur_val = root.as_ref().unwrap().borrow().val;
        match val.cmp(&cur_val) {
            Ordering::Less => {
                let left = root.as_ref().unwrap().borrow().left.clone();
                return Self::search_bst(left, val);
            },
            Ordering::Equal => return root,
            Ordering::Greater => {
                let right = root.as_ref().unwrap().borrow().right.clone();
                return Self::search_bst(right, val);
            },
        }
    }
}