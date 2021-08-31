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
use std::mem::swap;
impl Solution {
    pub fn recover_tree(root: &mut Option<Rc<RefCell<TreeNode>>>) {
        let mut v:Vec<Rc<RefCell<TreeNode>>> = Vec::new();
        traversal(root,&mut v);
        //println!("{:?}",v);
        
        let mut swap_a:Option<Rc<RefCell<TreeNode>>>=None;
        let mut swap_b:Option<Rc<RefCell<TreeNode>>>=None;
        for pairs in v.windows(2) {
            if swap_a.is_none() && pairs[0].borrow().val>=pairs[1].borrow().val{
                swap_a = Some(Rc::clone(&pairs[0]));
            }
            if pairs[0].borrow().val>=pairs[1].borrow().val{
                swap_b = Some(Rc::clone(&pairs[1]));
            }
        }
        swap(&mut swap_a.unwrap().borrow_mut().val, &mut swap_b.unwrap().borrow_mut().val);
    }
}

fn traversal<'a>(root: &'a Option<Rc<RefCell<TreeNode>>>, v:&'a mut Vec<Rc<RefCell<TreeNode>>>){
    if root.is_none(){
        return;
    }
    let mut r = root.as_ref().unwrap().borrow();
    let left:&Option<Rc<RefCell<TreeNode>>> = &r.left;
    traversal(left,v);
    //println!("{}", r.val);
    v.push(Rc::clone(root.as_ref().unwrap()));
    let right:&Option<Rc<RefCell<TreeNode>>> = &r.right;
    traversal(right,v);
}