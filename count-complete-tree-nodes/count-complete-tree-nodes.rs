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
    pub fn count_nodes(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        if root.is_none(){
            return 0;
        }
        let mut queue: VecDeque<Option<Rc<RefCell<TreeNode>>>> = VecDeque::new();
        queue.push_back(root);
        let mut count = 0;
        while !queue.is_empty(){
            let len = queue.len();
            count += len;
            for _ in (0..len){
                let node = queue.pop_front().unwrap();
                let left = node.as_ref().unwrap().borrow_mut().left.take();
                let right = node.as_ref().unwrap().borrow_mut().right.take();
                if left.is_some(){
                    queue.push_back(left);
                }
                if right.is_some(){
                    queue.push_back(right);
                }
            }
        }
        count as i32
    }
}