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
    pub fn kth_smallest(root: Option<Rc<RefCell<TreeNode>>>, mut k: i32) -> i32 {
        let mut stack = vec![];
        stack.push(root);
        
        while !stack.is_empty(){
            let root = stack.pop().unwrap();
            let val = root.as_ref().unwrap().borrow().val;
            let left = root.as_ref().unwrap().borrow_mut().left.take();
            let right = root.as_ref().unwrap().borrow_mut().right.take();
            
            match (left,right){
                (None, None) => k-=1,
                (None, x) => {
                    stack.push(x);
                    k-=1;
                },
                (x, None) => {
                    stack.push(root);
                    stack.push(x);
                },
                (left, right) => {
                    stack.push(right);
                    stack.push(root);
                    stack.push(left);
                },
            }
            
            if k == 0{
                return val;
            }
        }
        i32::MIN
    }
}