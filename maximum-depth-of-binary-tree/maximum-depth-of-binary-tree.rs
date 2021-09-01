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
    pub fn max_depth(mut root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        if root.is_none(){
            return 0;
        }
        let mut stack:Vec<(i32, Option<Rc<RefCell<TreeNode>>>)> = Vec::new();
        let mut depth:i32 = 1;
        let mut max_depth:i32 = 1;
        while root.is_some(){            
            let left = root.as_ref().unwrap().borrow().left.clone();
            let right = root.as_ref().unwrap().borrow().right.clone();
            
            if left.is_some(){
                stack.push((depth+1,left));    
            }
            if right.is_some(){
                stack.push((depth+1,right));    
            }
            
            let pairs = stack.pop();
            if pairs.is_none(){
                break;
            }
            depth = pairs.as_ref().unwrap().0;
            root = pairs.unwrap().1;
            max_depth = max_depth.max(depth);
        }
        max_depth
    }
}