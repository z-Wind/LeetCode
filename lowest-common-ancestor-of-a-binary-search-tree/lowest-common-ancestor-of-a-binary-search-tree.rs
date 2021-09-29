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
    if val == p || val == q{
        return root;
    }
    let left = root.as_ref().unwrap().borrow().left.clone();
    let right = root.as_ref().unwrap().borrow().right.clone();
    
    match (p<val, q<val){
        (true,true) => {
            if left.as_ref().unwrap().borrow().val == p
            || left.as_ref().unwrap().borrow().val == q{
                return left;
            }
            return lowest_common_ancestor(left, p, q);
        },
        (false,false) => {
            if right.as_ref().unwrap().borrow().val == p
            || right.as_ref().unwrap().borrow().val == q{
                return right;
            }
            return lowest_common_ancestor(right, p, q);
        },
        (true,false) | (false,true) =>{
            return root;
        },
    }
}