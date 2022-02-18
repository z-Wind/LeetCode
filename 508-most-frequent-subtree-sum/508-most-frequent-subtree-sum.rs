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
    pub fn find_frequent_tree_sum(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let mut map = HashMap::new();
        tree_sum(root.as_ref(), &mut map);

        let max_freq = map.values().max().cloned().unwrap();
        map.into_iter()
            .filter(|(k, v)| *v == max_freq)
            .map(|(k, _)| k)
            .collect()
    }
}

fn tree_sum(root: Option<&Rc<RefCell<TreeNode>>>, map: &mut HashMap<i32, i32>) -> i32 {
    if root.is_none() {
        return 0;
    }

    let root = root.unwrap().borrow();
    let val = root.val;
    let left = root.left.as_ref();
    let right = root.right.as_ref();
    let sum = val + tree_sum(left, map) + tree_sum(right, map);
    *map.entry(sum).or_insert(0) += 1;

    return sum;
}
