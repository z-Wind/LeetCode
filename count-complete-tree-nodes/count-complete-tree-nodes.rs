//https://leetcode.com/problems/count-complete-tree-nodes/discuss/61958/Concise-Java-solutions-O(log(n)2)
//
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
    pub fn count_nodes(mut root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut nodes = 0;
        let mut h = height(&root);
        while let Some(node) = root {
            if height(&node.borrow().right) == h - 1 {
                nodes += 1 << h; // 1*(2^(h-1) - 1)/(2-1) + 1 = 2^(h-1) = 1 << h
                root = node.borrow_mut().right.take();
            } else {
                nodes += 1 << h-1; // 1*(2^(h-2) - 1)/(2-1) + 1 = 2^(h-2) = 1 << (h-1)
                root = node.borrow_mut().left.take();
            }
            h-=1;
        }
        return nodes;
    }
}

fn height(root: &Option<Rc<RefCell<TreeNode>>>) -> i32 {
    match root{
        None => -1,
        Some(root) => 1 + height(&root.borrow().left),
    }
}
