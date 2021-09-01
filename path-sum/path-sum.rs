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
    pub fn has_path_sum(root: Option<Rc<RefCell<TreeNode>>>, mut target_sum: i32) -> bool {
        if root.is_none(){
            return false;
        }

        target_sum -= root.as_ref().unwrap().borrow().val;       

        let left = root.as_ref().unwrap().borrow().left.clone();
        let right = root.as_ref().unwrap().borrow().right.clone();

        match (left,right){
            (None,None) => target_sum == 0,
            (Some(x),None) | (None,Some(x)) => Self::has_path_sum(Some(x), target_sum),
            (Some(l),Some(r)) => Self::has_path_sum(Some(l), target_sum) || Self::has_path_sum(Some(r), target_sum),
        }
    }
}