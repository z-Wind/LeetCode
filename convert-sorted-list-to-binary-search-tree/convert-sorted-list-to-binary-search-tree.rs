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
        build_balance_tree(&head, &None)
    }
}

fn build_balance_tree(head: &Option<Box<ListNode>>, tail: &Option<Box<ListNode>>) -> Option<Rc<RefCell<TreeNode>>>{
    if head.is_none() || (tail.is_some() && head.as_ref().unwrap().val == tail.as_ref().unwrap().val){
        return None;
    }
    
    // find mid element
    let mut double:&Option<Box<ListNode>> = head;
    let mut single:&Option<Box<ListNode>> = head;
    while (tail.is_none() && (double.is_some() && double.as_ref().unwrap().next.is_some())) || (
    (double.is_some() && tail.is_some() && double.as_ref().unwrap().val != tail.as_ref().unwrap().val) &&
          (double.as_ref().unwrap().next.is_some() && tail.is_some() && double.as_ref().unwrap().next.as_ref().unwrap().val != tail.as_ref().unwrap().val)) {
        double = &double.as_ref().unwrap().next.as_ref().unwrap().next;
        single = &single.as_ref().unwrap().next;
    }
    
    let mut root = TreeNode::new(single.as_ref().unwrap().val);
    root.left = build_balance_tree(head, single);
    root.right = build_balance_tree(&single.as_ref().unwrap().next, tail);
    Some(Rc::new(RefCell::new(root)))
}

