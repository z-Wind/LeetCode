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
struct BSTIterator {
    nodes: Vec::<Rc<RefCell<TreeNode>>>,
}


/** 
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl BSTIterator {

    fn new(mut root: Option<Rc<RefCell<TreeNode>>>) -> Self {
        let mut nodes = vec![];
        while let Some(node) = root{
            root = node.borrow_mut().left.take();
            nodes.push(node);
        }
        BSTIterator{
            nodes,
        }
    }
    
    fn next(&mut self) -> i32 {
        if !self.has_next(){
            panic!("No next");
        }
        let node = self.nodes.pop().unwrap();
        let mut root = node.borrow_mut().right.take();
        while let Some(node) = root{
            root = node.borrow_mut().left.take();
            self.nodes.push(node);
        }
        return node.borrow().val;
    }
    
    fn has_next(&self) -> bool {
        self.nodes.len() != 0
    }
}

/**
 * Your BSTIterator object will be instantiated and called as such:
 * let obj = BSTIterator::new(root);
 * let ret_1: i32 = obj.next();
 * let ret_2: bool = obj.has_next();
 */