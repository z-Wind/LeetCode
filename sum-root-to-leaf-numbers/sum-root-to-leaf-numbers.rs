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
    pub fn sum_numbers(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let (_,sum) = sum_numbers(root);
        sum
    }
}

fn sum_numbers(root: Option<Rc<RefCell<TreeNode>>>) -> (Vec<i32>, i32) {
    let val = root.as_ref().unwrap().borrow().val;
    let left = root.as_ref().unwrap().borrow().left.clone();
    let right = root.as_ref().unwrap().borrow().right.clone();
    match (left,right){
        (None,None) => (vec![0],val),
        (Some(x),None) | (None,Some(x)) => {
            let (mut layers, mut sum) = sum_numbers(Some(x));
            for layer in layers.iter_mut(){
                sum += val * 10_i32.pow((*layer+1) as u32);
                *layer += 1
            }           
            (layers, sum)
        },
        (Some(l), Some(r)) => {
            let (mut layer_l, sum_l) = sum_numbers(Some(l));
            let (mut layer_r, sum_r) = sum_numbers(Some(r));
            layer_l.append(&mut layer_r);
            let mut layers = layer_l;
            let mut sum = sum_l+sum_r;
            for layer in layers.iter_mut(){
                sum += val * 10_i32.pow((*layer+1) as u32);
                *layer += 1
            }           
            (layers, sum)
        },
    }
}