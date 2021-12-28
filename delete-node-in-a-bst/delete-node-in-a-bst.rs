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
use std::cmp::Ordering;

impl Solution {
    pub fn delete_node(mut root: Option<Rc<RefCell<TreeNode>>>, key: i32) -> Option<Rc<RefCell<TreeNode>>> {
        delete_node(&mut root, key);
        root
    }
}

fn delete_node(root: &mut Option<Rc<RefCell<TreeNode>>>, key: i32) {
    if root.is_none(){
        return;
    }
    
    let val = root.as_ref().unwrap().borrow().val;
    // println!("{}", val);

    match key.cmp(&val){
        Ordering::Less => {
            delete_node(&mut root.as_ref().unwrap().borrow_mut().left, key);
        },
        Ordering::Greater => {
            delete_node(&mut root.as_ref().unwrap().borrow_mut().right, key);
        },
        Ordering::Equal => {
            let mut left = root.as_ref().unwrap().borrow_mut().left.take();
            let mut right = root.as_ref().unwrap().borrow_mut().right.take();
            *root = combine_tree(left, right);
        },
    }    
}

fn combine_tree(mut left: Option<Rc<RefCell<TreeNode>>>, mut right: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
    if right.is_none(){
        return left;
    } else if left.is_none(){
        return right;
    }
    
    let mut r = left.as_ref().unwrap().borrow_mut().right.take();
    left.as_ref().unwrap().borrow_mut().right = combine_tree(r, right);
    
    left
}