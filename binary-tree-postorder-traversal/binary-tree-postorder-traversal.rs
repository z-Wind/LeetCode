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
    pub fn postorder_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {     
        if root.is_none(){
            return vec![];
        }
        let mut ans:Vec<i32> = Vec::new();
        let mut stack:Vec<Option<Rc<RefCell<TreeNode>>>> = Vec::new();
        stack.push(root);
        while !stack.is_empty(){
            let root = stack.pop().unwrap();
            let left = root.as_ref().unwrap().borrow_mut().left.take();
            let right = root.as_ref().unwrap().borrow_mut().right.take();
            match (left, right){
                (None, None) => {
                    let val = root.as_ref().unwrap().borrow().val;
                    //println!("{}",val);
                    ans.push(val);
                }
                (Some(x),None) | (None, Some(x)) => {
                    stack.push(root);
                    stack.push(Some(x));    
                }
                (Some(l),Some(r)) => {
                    stack.push(root);
                    stack.push(Some(r));    
                    stack.push(Some(l));    
                }
            }
        }
        
        
        ans
    }
}

fn postorder_traversal(ans: &mut Vec<i32>,root: Option<Rc<RefCell<TreeNode>>>){
    if root.is_none(){
        return;
    }
    
    let val = root.as_ref().unwrap().borrow().val;
    // println!("{}",val);
    let left = root.as_ref().unwrap().borrow_mut().left.take();
    let right = root.as_ref().unwrap().borrow_mut().right.take();
    
    postorder_traversal(ans, left);
    postorder_traversal(ans, right);
    ans.push(val);    
}