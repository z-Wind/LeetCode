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
    pub fn convert_bst(root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
        let mut sum = 0;
        convert_bst(root.clone(), &mut sum);
        root
    }
}

fn convert_bst(root: Option<Rc<RefCell<TreeNode>>>, sum:&mut i32)  {
    match root {
        None => (),
        Some(node) => {
            convert_bst(node.borrow().right.clone(), sum);
            *sum += node.borrow().val;
            node.borrow_mut().val = *sum;
            convert_bst(node.borrow().left.clone(), sum);
        }
    }
}