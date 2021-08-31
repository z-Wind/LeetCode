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
// https://leetcode.com/problems/recover-binary-search-tree/discuss/32559/Detail-Explain-about-How-Morris-Traversal-Finds-two-Incorrect-Pointer
use std::cell::RefCell;
use std::mem::swap;
use std::rc::Rc;
impl Solution {
    pub fn recover_tree(root: &mut Option<Rc<RefCell<TreeNode>>>) {
        let mut root: Option<Rc<RefCell<TreeNode>>> = root.clone();
        let mut pre: Option<Rc<RefCell<TreeNode>>> = None;
        let mut first: Option<Rc<RefCell<TreeNode>>> = None;
        let mut second: Option<Rc<RefCell<TreeNode>>> = None;
        // Morris Traversal
        let mut temp: Option<Rc<RefCell<TreeNode>>> = None;
        while root.is_some() {
            if root.as_ref().unwrap().borrow().left.is_some() {
                // connect threading for root
                temp = root.as_ref().unwrap().borrow().left.clone();
                while (temp.as_ref().unwrap().borrow().right.is_some()
                    && temp.as_ref().unwrap().borrow().right.as_ref().unwrap().borrow().val != root.as_ref().unwrap().borrow().val)
                {
                    let tt = temp.as_ref().unwrap().borrow().right.clone();
                    temp = tt;
                }
                // the threading already exists
                if temp.as_ref().unwrap().borrow().right.is_some() {
                    if pre.is_some()
                        && pre.as_ref().unwrap().borrow().val > root.as_ref().unwrap().borrow().val
                    {
                        if first.is_none() {
                            first = pre.clone();
                            second = root.clone();
                        } else {
                            second = root.clone();
                        }
                    }
                    pre = root.clone();

                    temp.as_ref().unwrap().borrow_mut().right = None;
                    let rr = root.as_ref().unwrap().borrow().right.clone();
                    root = rr;
                } else {
                    // construct the threading
                    temp.as_ref().unwrap().borrow_mut().right = root.clone();
                    let rr = root.as_ref().unwrap().borrow().left.clone();
                    root = rr;
                }
            } else {
                if pre.is_some()
                    && pre.as_ref().unwrap().borrow().val > root.as_ref().unwrap().borrow().val
                {
                    if first.is_none() {
                        first = pre.clone();
                        second = root.clone();
                    } else {
                        second = root.clone();
                    }
                }
                pre = root.clone();
                let rr = root.as_ref().unwrap().borrow().right.clone();
                root = rr;
            }
        }
        // swap two node values;
        if (first.is_some() && second.is_some()) {
            swap(&mut first.as_ref().unwrap().borrow_mut().val, &mut second.as_ref().unwrap().borrow_mut().val);
        }
    }
}
