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
    pub fn invert_tree(root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
        if root.is_none(){
            return None;
        }
        let left = root.as_ref().unwrap().borrow_mut().left.take();
        let right = root.as_ref().unwrap().borrow_mut().right.take();
        
        root.as_ref().unwrap().borrow_mut().right = Self::invert_tree(left);
        root.as_ref().unwrap().borrow_mut().left = Self::invert_tree(right);
        root
    }
}