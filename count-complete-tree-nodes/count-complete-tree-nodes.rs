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
    pub fn count_nodes(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        if root.is_none(){
            return 0;
        }
        
        let (check, h) = is_complete(&root);
        // println!("{:?}",(check, h));
        if check{
            return 2_i32.pow(h)-1;
        }
        
        let mut count = 1;
        let left = root.as_ref().unwrap().borrow_mut().left.take();
        let right = root.as_ref().unwrap().borrow_mut().right.take();
        
        let (check, h) = is_complete(&left);
        if check{
            count += 2_i32.pow(h)-1;
        } else {
            count += Self::count_nodes(left);
        }
        
        let (check, h) = is_complete(&right);
        if check{
            count += 2_i32.pow(h)-1;
        } else {
            count += Self::count_nodes(right);
        }
        count
    }
}

fn is_complete(root: &Option<Rc<RefCell<TreeNode>>>) -> (bool,u32){
    if root.is_none(){
        return (true,0);
    }
    let left = &root.as_ref().unwrap().borrow().left;
    let right = &root.as_ref().unwrap().borrow().right;
    match (left, right){
        (None,None) => (true, 1),
        (x,None) => (false,0),
        (left, right) => {
            let (check_left, h_left) = is_complete(left);
            let (check_right, h_right) = is_complete(right);
            // println!("{:?}, {:?}",(check_left, h_left),(check_right, h_right));
            if check_left && check_right && h_left == h_right{
                (true, h_left+1)
            } else {
                (false, 0)
            }
        },
        (None,x) => panic!(),
    }
}