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
    pub fn sorted_array_to_bst(nums: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
        build_balance_tree(&nums[..])
    }
}

fn build_balance_tree(nums: &[i32]) -> Option<Rc<RefCell<TreeNode>>>{
    if nums.is_empty(){
        return None;
    }
    let mid = nums.len()/2;
    let mut root = TreeNode::new(nums[mid]);
    root.left = build_balance_tree(&nums[..mid]);
    root.right = build_balance_tree(&nums[mid+1..]);
    Some(Rc::new(RefCell::new(root)))
}