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

use std::collections::VecDeque;
impl Solution {
    pub fn zigzag_level_order(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>> {
        let mut ans:Vec<Vec<i32>> = Vec::new();
        if root.is_none(){
            return ans;
        }
        
        let mut deque:VecDeque<Rc<RefCell<TreeNode>>> = VecDeque::new();
        deque.push_back(Rc::clone(root.as_ref().unwrap()));
        let mut reverse = false;
        while !deque.is_empty(){
            let mut temp:Vec<i32> = Vec::new();
            
            let len = deque.len();
            for _ in (0..len){
                let root = deque.pop_front().unwrap();
                
                temp.push(root.borrow().val);
                
                let left = root.borrow().left.clone();
                let right = root.borrow().right.clone(); 
                
                if root.borrow().left.is_some(){
                    deque.push_back((Rc::clone(root.borrow().left.as_ref().unwrap())));    
                }
                if root.borrow().right.is_some(){
                    deque.push_back((Rc::clone(root.borrow().right.as_ref().unwrap())));    
                }
            }
            // println!("{:?}", temp);
            if reverse{
                temp.reverse();
            }
            ans.push(temp);
            reverse = !reverse;
        }
        ans
    }
}