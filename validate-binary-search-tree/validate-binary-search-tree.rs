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
        is_valid_bst(&root,None,None)
    }
}

fn get_val(node: &Option<Rc<RefCell<TreeNode>>>) -> i32{
    node.as_ref().unwrap().borrow().val
}

fn is_valid_bst(root: &Option<Rc<RefCell<TreeNode>>>, lowerbound:Option<i32>, upperbound:Option<i32>) -> bool {
    if root.is_none(){
        return true;
    }
    
    let val = get_val(root);  
    if (upperbound.is_some() && val >= upperbound.unwrap()) || 
       (lowerbound.is_some() && val <= lowerbound.unwrap()){
        return false;
    }
     
    let left:&Option<Rc<RefCell<TreeNode>>> = &root.as_ref().unwrap().borrow().left;
    let right:&Option<Rc<RefCell<TreeNode>>> = &root.as_ref().unwrap().borrow().right;
    
    if left.is_some() && val <= get_val(left){
        return false
    } else if right.is_some() && val >= get_val(right){
        return false
    }
    
    is_valid_bst(left, lowerbound, Some(val)) && 
    is_valid_bst(right, Some(val), upperbound)   
}