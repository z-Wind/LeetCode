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
    if root.is_none(){
        return None;
    }
    
    let val = root.as_ref().unwrap().borrow().val;
    if val == p || val == q{
        return root; 
    }

    let left = root.as_ref().unwrap().borrow_mut().left.take();
    let right = root.as_ref().unwrap().borrow_mut().right.take();

    match (lowest_common_ancestor(left, p, q), lowest_common_ancestor(right, p, q)){
        (Some(_),Some(_)) => root,
        (None,x) | (x,None) => x,
        (None,None) => None,
    }
}