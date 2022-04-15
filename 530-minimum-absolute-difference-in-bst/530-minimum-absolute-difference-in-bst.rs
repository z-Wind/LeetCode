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
use std::rc::Rc;
impl Solution {
    pub fn get_minimum_difference(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut prev_val = None;
        let mut ans = i32::MAX;
        get_minimum_difference(&mut ans, &mut prev_val, root);
        ans
    }
}

// inorder
fn get_minimum_difference(
    ans: &mut i32,
    prev_val: &mut Option<i32>,
    root: Option<Rc<RefCell<TreeNode>>>,
) {
    if root.is_none() {
        return;
    }
    
    let left = root.as_ref().unwrap().borrow().left.clone();
    get_minimum_difference(ans, prev_val, left);

    let val = root.as_ref().unwrap().borrow().val;
    if let Some(x) = prev_val {
        *ans = (*ans).min((val - *x).abs());
    }
    *prev_val = Some(val);

    let right = root.as_ref().unwrap().borrow().right.clone();
    get_minimum_difference(ans, prev_val, right);
}
