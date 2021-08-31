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
    pub fn is_valid_bst(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        let mut prev:Option<i32> = None;
        is_valid_bst(&root,&mut prev)
    }
}

fn get_val(node: &Option<Rc<RefCell<TreeNode>>>) -> i32{
    node.as_ref().unwrap().borrow().val
}

fn is_valid_bst<'a>(root: &'a Option<Rc<RefCell<TreeNode>>>, prev: &'a mut Option<i32>) -> bool {
    if root.is_none(){
        return true;
    }
    let left:&Option<Rc<RefCell<TreeNode>>> = &root.as_ref().unwrap().borrow().left;
    if !is_valid_bst(left,prev){
        return false;
    }
    if prev.is_some() && prev.unwrap() >= get_val(root){
        return false
    }
    *prev = Some(get_val(root));
    let right:&Option<Rc<RefCell<TreeNode>>> = &root.as_ref().unwrap().borrow().right;
    return is_valid_bst(right,prev);
}