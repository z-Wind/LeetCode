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
    } else if (upperbound.is_some() && get_val(root) >= upperbound.unwrap()) || 
              (lowerbound.is_some() && get_val(root) <= lowerbound.unwrap()){
        return false;
    }
    let val = get_val(root);   
    let left:&Option<Rc<RefCell<TreeNode>>> = &root.as_ref().unwrap().borrow().left;
    let right:&Option<Rc<RefCell<TreeNode>>> = &root.as_ref().unwrap().borrow().right;
    
    if left.is_some() && val <= get_val(left){
        return false
    } else if right.is_some() && val >= get_val(right){
        return false
    }
    //println!("root:{}, lowerbound:{}, upperbound:{}, left:{:?}, right:{:?}",get_val(root),lowerbound,upperbound,left,right);
    match (lowerbound, upperbound){
        (None,None) => is_valid_bst(left, None, Some(val)) && 
                       is_valid_bst(right, Some(val), None),
        (Some(l),None) => is_valid_bst(left, Some(l), Some(val)) && 
                          is_valid_bst(right, Some(val), None),
        (None,Some(u)) => is_valid_bst(left, None, Some(val)) && 
                          is_valid_bst(right, Some(val), Some(u)),
        (Some(l),Some(u)) => is_valid_bst(left, Some(l), Some(val)) && 
                             is_valid_bst(right, Some(val), Some(u)),
    }
}