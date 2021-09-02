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
        let mut flat = Flat::new();
        flat.flatten(root);        
    }
}

struct Flat{
    pre:Option<Rc<RefCell<TreeNode>>>,
}

impl Flat{
    fn new() -> Self{
        Flat{
            pre:None,
        }
    }
    fn flatten(&mut self, root: &Option<Rc<RefCell<TreeNode>>>) {
        if root.is_none(){
            return;
        }
        
        let left = root.as_ref().unwrap().borrow_mut().left.take();
        let right = root.as_ref().unwrap().borrow_mut().right.take();
        
        self.flatten(&right);
        self.flatten(&left);
        
        root.as_ref().unwrap().borrow_mut().right = self.pre.clone();
        self.pre = root.clone();        
    }
}