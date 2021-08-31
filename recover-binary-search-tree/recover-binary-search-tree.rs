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
    pub fn recover_tree(root: &mut Option<Rc<RefCell<TreeNode>>>) {
        let mut v:Vec<Rc<RefCell<TreeNode>>> = Vec::new();
        traversal(root,&mut v);
        //println!("{:?}",v);
        for i in (0..v.len()){
            let mut min = v[i].borrow().val;
            for j in (i+1..v.len()){
                if v[j].borrow().val < min{
                    let temp = min;
                    min = v[j].borrow().val;
                    v[i].borrow_mut().val = min;
                    v[j].borrow_mut().val = temp;
                }
            }
        }
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
    v.push(root.clone().unwrap());
    let right:&Option<Rc<RefCell<TreeNode>>> = &r.right;
    traversal(right,v);
}