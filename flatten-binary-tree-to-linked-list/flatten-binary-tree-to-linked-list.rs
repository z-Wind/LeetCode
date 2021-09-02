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
    pub fn flatten(root: &mut Option<Rc<RefCell<TreeNode>>>) {
        if root.is_none(){
            return;
        }
        
        let mut root:Option<Rc<RefCell<TreeNode>>> = root.clone();
        let mut stack: Vec<Option<Rc<RefCell<TreeNode>>>> = Vec::new();
        
        loop{
            //println!("{:?}", stack);
            let left = root.as_ref().unwrap().borrow_mut().left.take();
            let right = root.as_ref().unwrap().borrow_mut().right.take();
            
            match (left,right){
                (None, None) => {
                    if stack.is_empty(){
                        break;
                    }
                    let node = stack.pop().unwrap();
                    root.as_ref().unwrap().borrow_mut().right = node.clone();
                    root = node;
                },
                (l, None) => {
                    root.as_ref().unwrap().borrow_mut().right = l.clone();
                    root = l;
                },
                (None, r) =>{
                    root.as_ref().unwrap().borrow_mut().right = r.clone();
                    root = r;
                },
                (l, r) => {
                    stack.push(r);
                    root.as_ref().unwrap().borrow_mut().right = l.clone();
                    root = l;
                },
            }
        }
        
        
    }
}