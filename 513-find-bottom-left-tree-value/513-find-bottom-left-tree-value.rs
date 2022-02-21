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
        
        while !queue.is_empty() {
            // println!("{:?}", queue);
            ans = queue.front().unwrap().borrow().val;
            
            let n = queue.len();
            for _ in 0..n {
                let node = queue.pop_front().unwrap();
                let node = node.borrow();
                if let Some(left) = &node.left {
                    queue.push_back(left.clone());
                }
                if let Some(right) = &node.right {
                    queue.push_back(right.clone());
                }
            }    
        }
        
        ans
    }
}