// https://leetcode.com/problems/find-mode-in-binary-search-tree/discuss/98101/Proper-O(1)-space/273825

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
        let mut maxCount = i32::MIN;
        let mut currCount = 0;
        let mut pre = None;
        let mut ans = Vec::new();

        // Get the maxCount size
        inorder(false, root.clone(), &mut maxCount, &mut currCount, &mut pre, &mut ans);

        // Get the final list
        pre = None;
        currCount = 0;
        inorder(true, root.clone(), &mut maxCount, &mut currCount, &mut pre, &mut ans);

        return ans;
    }
}

// use inorder to get sorted order
fn inorder(getlist:bool, root: Option<Rc<RefCell<TreeNode>>>, maxCount:&mut i32, currCount:&mut i32, pre:&mut Option<i32>, ans:&mut Vec<i32>){
    if root.is_none() {
        return;
    }
    
    let val = root.as_ref().unwrap().borrow().val;
    let left = root.as_ref().unwrap().borrow().left.clone();
    let right = root.as_ref().unwrap().borrow().right.clone();

    inorder(getlist, left, maxCount, currCount, pre, ans);
    
    // extra action
    if pre.is_some() && pre.unwrap() == val {
        *currCount += 1;
    } else {
        *currCount = 1;
    }
    if !getlist { 
        *maxCount = (*maxCount).max(*currCount);
    } else if currCount == maxCount {
        ans.push(val);
    }
    *pre = Some(val);
    
    inorder(getlist, right, maxCount, currCount, pre, ans);
}
