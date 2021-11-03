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

// (1) root = root.children.children + root.money if robbing,
// (2) root = root.children           if not robbing
// root = max(root.children, root.children.children + root.money)
use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    pub fn rob(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let ans = rob(root.as_ref());
        ans.0.max(ans.1 + ans.2)
    }
}

fn rob(root: Option<&Rc<RefCell<TreeNode>>>) -> (i32, i32, i32) {
    if root.is_none() {
        return (0, 0, 0);
    }
    let root = root.as_ref().unwrap().borrow_mut();
    let left_money = rob(root.left.as_ref());
    let right_money = rob(root.right.as_ref());

    let ans = (
        root.val + left_money.1 + left_money.2 + right_money.1 + right_money.2,
        left_money.0.max(left_money.1+left_money.2),
        right_money.0.max(right_money.1+right_money.2),
    );
    // println!("{} => {:?}", root.val, ans);
    ans
}
