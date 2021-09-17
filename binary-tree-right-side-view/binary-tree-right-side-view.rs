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
        let mut result = Vec::new();
        rightView(root, &mut result, 0);
        result
    }
}

fn rightView(curr:Option<Rc<RefCell<TreeNode>>>, result:&mut Vec<i32>, currDepth:usize){
    if curr.is_none(){
        return;
    }
    if(currDepth == result.len()){
        result.push(curr.as_ref().unwrap().borrow().val);
    }

    rightView(curr.as_ref().unwrap().borrow_mut().right.take(), result, currDepth + 1);
    rightView(curr.as_ref().unwrap().borrow_mut().left.take(), result, currDepth + 1);
}