// https://leetcode.com/problems/delete-node-in-a-bst/discuss/1590868/JAVA-or-Recursive-or-Most-Intutive-or-Proper-Explanation-Using-Image

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
use std::cell::RefCell;
use std::cmp::Ordering;
use std::rc::Rc;

impl Solution {
    pub fn delete_node(
        root: Option<Rc<RefCell<TreeNode>>>,
        key: i32,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        if root.is_none() {
            return root;
        }

        let val = root.as_ref().unwrap().borrow().val;
        // println!("{}", val);

        match key.cmp(&val) {
            Ordering::Less => {
                let node = Self::delete_node(root.as_ref().unwrap().borrow_mut().left.take(), key);
                root.as_ref().unwrap().borrow_mut().left = node;
            }
            Ordering::Greater => {
                let node = Self::delete_node(root.as_ref().unwrap().borrow_mut().right.take(), key);
                root.as_ref().unwrap().borrow_mut().right = node;
            }
            Ordering::Equal => {
                if root.clone().as_ref().unwrap().borrow().left.is_none() {
                    return root.as_ref().unwrap().borrow_mut().right.take();
                }
                if root.clone().as_ref().unwrap().borrow().right.is_none() {
                    return root.as_ref().unwrap().borrow_mut().left.take();
                }

                let val = find_min(&root.as_ref().unwrap().borrow().right);
                root.as_ref().unwrap().borrow_mut().val = val;
                let node = Self::delete_node(root.as_ref().unwrap().borrow_mut().right.take(), val);
                root.as_ref().unwrap().borrow_mut().right = node;
                    
            }
        }
        root
    }
}

fn find_min(root: &Option<Rc<RefCell<TreeNode>>>) -> i32 {
    match root.as_ref().unwrap().borrow().left.clone() {
        None => root.as_ref().unwrap().borrow().val,
        node => find_min(&node),
    }
}
