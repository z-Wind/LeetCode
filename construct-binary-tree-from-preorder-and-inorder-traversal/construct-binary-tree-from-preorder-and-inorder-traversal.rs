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
    pub fn build_tree(preorder: Vec<i32>, inorder: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
        build_tree(&preorder[..], &inorder[..])
    }
}

fn build_tree(preorder: &[i32], inorder: &[i32]) -> Option<Rc<RefCell<TreeNode>>> {
    if preorder.is_empty(){
        return None;
    }
    let mut root = TreeNode::new(preorder[0]);
    let i = inorder.iter().position(|&x| x == preorder[0]).unwrap();
    
    root.left = build_tree(&preorder[1..i+1], &inorder[0..i]);
    root.right = build_tree(&preorder[i+1..], &inorder[i+1..]);
    Some(Rc::new(RefCell::new(root)))
}