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
    pub fn largest_values(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        if root.is_none() {
            return Vec::new();
        }
        
        let mut queue:VecDeque<Rc<RefCell<TreeNode>>> = VecDeque::new();
        let mut ans = Vec::new();
        queue.push_back(root.unwrap());
        
        while !queue.is_empty() {
            // println!("{:?}", queue);
            let mut max = i32::MIN;
            let n = queue.len();
            for _ in 0..n {
                let node = queue.pop_front().unwrap();
                let node = node.borrow();
                max = max.max(node.val);
                if let Some(left) = &node.left {
                    queue.push_back(left.clone());
                }
                if let Some(right) = &node.right {
                    queue.push_back(right.clone());
                }
            }    
            
            ans.push(max);
        }
        
        ans
    }
}