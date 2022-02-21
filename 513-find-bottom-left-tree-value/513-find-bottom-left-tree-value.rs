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
use std::collections::VecDeque;

impl Solution {
    pub fn find_bottom_left_value(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut queue:VecDeque<Rc<RefCell<TreeNode>>> = VecDeque::new();
        let mut ans = root.as_ref().unwrap().borrow().val;
        queue.push_back(root.unwrap());
        
        while let Some(node) = queue.pop_front() {
            ans = node.borrow().val;
            
            if let Some(right) = &node.borrow().right {
                queue.push_back(right.clone());
            }    
            if let Some(left) = &node.borrow().left {
                queue.push_back(left.clone());
            }
        }
        
        ans
    }
}