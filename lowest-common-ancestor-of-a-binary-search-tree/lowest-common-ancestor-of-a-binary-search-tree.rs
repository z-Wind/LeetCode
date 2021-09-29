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
    pub fn lowest_common_ancestor(root: Option<Rc<RefCell<TreeNode>>>, p: Option<Rc<RefCell<TreeNode>>>, q: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
        let p_val = p.as_ref().unwrap().borrow().val;
        let q_val = q.as_ref().unwrap().borrow().val;
    
        lowest_common_ancestor(root, p_val, q_val)
    }
}

fn lowest_common_ancestor(root: Option<Rc<RefCell<TreeNode>>>, p:i32, q:i32) -> Option<Rc<RefCell<TreeNode>>> {
    let val = root.as_ref().unwrap().borrow().val;
    
    if p<val && q<val{
        lowest_common_ancestor(root.unwrap().borrow_mut().left.take(), p, q)
    } else if p>val && q>val{
        lowest_common_ancestor(root.unwrap().borrow_mut().right.take(), p, q)
    } else {
        root
    }
}