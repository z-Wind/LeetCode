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
    pub fn level_order(mut root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>> {
        let mut ans:Vec<Vec<i32>> = Vec::new();
        let mut deque:VecDeque<(usize, Option<Rc<RefCell<TreeNode>>>)> = VecDeque::new();
        let mut layer = 0;
        while root.is_some(){
            if ans.len() < layer+1{
                ans.push(vec![root.as_ref().unwrap().borrow().val]);
            } else {
                ans[layer].push(root.as_ref().unwrap().borrow().val);
            }
            
            let left = root.as_ref().unwrap().borrow().left.clone();
            let right = root.as_ref().unwrap().borrow().right.clone();
            
            if left.is_some(){
                deque.push_back((layer+1,left));    
            }
            if right.is_some(){
                deque.push_back((layer+1,right));    
            }
            
            let pairs = deque.pop_front();
            if pairs.is_none(){
                break;
            }
            layer = pairs.as_ref().unwrap().0;
            root = pairs.unwrap().1;
        }
        ans
    }
}