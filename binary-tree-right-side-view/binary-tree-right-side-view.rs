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
    pub fn right_side_view(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        if root.is_none(){
            return vec![];
        }
        let mut queue = VecDeque::new();
        queue.push_back(root);
        let mut ans = Vec::new();
        while !queue.is_empty(){
            let len = queue.len();
            for i in (0..len){
                let root = queue.pop_front().unwrap();
                if i == len-1{
                    ans.push(root.as_ref().unwrap().borrow().val)
                }
                let left = root.as_ref().unwrap().borrow_mut().left.take();
                if left.is_some(){
                    queue.push_back(left);
                }
                let right = root.as_ref().unwrap().borrow_mut().right.take();
                if right.is_some(){
                    queue.push_back(right);
                }
            }
        }
        ans
    }
}