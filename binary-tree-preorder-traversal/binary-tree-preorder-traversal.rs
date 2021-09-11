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
    pub fn preorder_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let mut ans:Vec<i32> = Vec::new();
        preorder_traversal(&mut ans, root);
        ans
    }
}

fn preorder_traversal(ans: &mut Vec<i32>,root: Option<Rc<RefCell<TreeNode>>>){
    if root.is_none(){
        return;
    }
    
    let val = root.as_ref().unwrap().borrow().val;
    // println!("{}",val);
    let left = root.as_ref().unwrap().borrow_mut().left.take();
    let right = root.as_ref().unwrap().borrow_mut().right.take();
    
    ans.push(val);
    preorder_traversal(ans, left);
    preorder_traversal(ans, right);
}