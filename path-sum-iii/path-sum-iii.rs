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

// https://leetcode.com/problems/path-sum-iii/discuss/91878/17-ms-O(n)-java-Prefix-sum-method/96424

use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;
impl Solution {
    pub fn path_sum(root: Option<Rc<RefCell<TreeNode>>>, target_sum: i32) -> i32 {
        if root.is_none() {
            return 0;
        }
        // The map stores <prefix sum, frequency> pairs before getting to the current node. 
        // We can imagine a path from the root to the current node. 
        // The sum from any node in the middle of the path to the current node = the difference between the sum from the root to the current node and the prefix sum of the node in the middle.
        let mut map: HashMap<i32, i32> = HashMap::new();
        map.insert(0, 1);
        find_path_sum(root.as_ref(), 0, target_sum, &mut map)
    }
}

fn find_path_sum(
    curr: Option<&Rc<RefCell<TreeNode>>>,
    mut sum: i32,
    target: i32,
    map: &mut HashMap<i32, i32>,
) -> i32 {
    if curr.is_none() {
        return 0;
    }
    // update the prefix sum by adding the current val
    sum += curr.unwrap().borrow().val;
    
    // get the number of valid path, ended by the current node
    let numPathToCurr = *map.get(&(sum - target)).unwrap_or(&0);
    
    // update the map with the current sum, so the map is good to be passed to the next recursion
    *map.entry(sum).or_insert(0) += 1;
    
    // add the 3 parts discussed in 8. together
    let res = numPathToCurr
        + find_path_sum(curr.unwrap().borrow().left.as_ref(), sum, target, map)
        + find_path_sum(curr.unwrap().borrow().right.as_ref(), sum, target, map);
    
    // restore the map, as the recursion goes from the bottom to the top
    *map.entry(sum).or_insert(0) -= 1;

    return res;
}
