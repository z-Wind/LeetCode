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
use std::collections::HashMap;
use std::rc::Rc;
impl Solution {
    pub fn find_mode(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let mut map = HashMap::new();
        find_mode(root, &mut map);

        let freq = map.values().max().cloned().unwrap();
        map.into_iter()
            .filter(|(_, v)| *v == freq)
            .map(|(k, _)| k)
            .collect()
    }
}

fn find_mode(root: Option<Rc<RefCell<TreeNode>>>, map: &mut HashMap<i32, i32>) {
    if root.is_none() {
        return;
    }

    let val = root.as_ref().unwrap().borrow().val;
    let left = root.as_ref().unwrap().borrow().left.clone();
    let right = root.as_ref().unwrap().borrow().right.clone();
    *map.entry(val).or_insert(0) += 1;
    find_mode(left, map);
    find_mode(right, map);
}
