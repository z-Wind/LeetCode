// Definition for singly-linked list.
// #[derive(PartialEq, Eq, Clone, Debug)]
// pub struct ListNode {
//   pub val: i32,
//   pub next: Option<Box<ListNode>>
// }
// 
// impl ListNode {
//   #[inline]
//   fn new(val: i32) -> Self {
//     ListNode {
//       next: None,
//       val
//     }
//   }
// }
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
    pub fn sorted_list_to_bst(head: Option<Box<ListNode>>) -> Option<Rc<RefCell<TreeNode>>> {
        let mut nums:Vec<i32> = Vec::new();
        let mut h = head;
        while let Some(node) = h{
            nums.push(node.val);
            h = node.next;
        }
        // println!("{:?}", nums);
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