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
    pub fn min_depth(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        if root.is_none(){
            return 0;
        }
        let mut queue: VecDeque<Option<Rc<RefCell<TreeNode>>>> = VecDeque::new();
        queue.push_back(root);
        let mut depth = 0;
        'outer: while !queue.is_empty(){
            depth += 1;
            let len = queue.len();
            for _ in (0..len){
                let mut root = queue.pop_front().unwrap();
                let left = root.as_ref().unwrap().borrow().left.clone();
                let right = root.as_ref().unwrap().borrow().right.clone();
                if left.is_none() && right.is_none(){
                    break 'outer;
                }
                if left.is_some(){
                    queue.push_back(left);
                }
                if right.is_some(){
                    queue.push_back(right);
                }
            }
        }
        depth
    }
}