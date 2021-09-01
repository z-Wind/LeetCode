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
use std::collections::HashMap;
impl Solution {
    pub fn build_tree(preorder: Vec<i32>, inorder: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
        let mut inorder_map:HashMap<i32,usize> = HashMap::new();
        for (i,val) in inorder.into_iter().enumerate(){
            inorder_map.insert(val,i);
        }
        build_tree(&preorder[..], 0, preorder.len()-1, &inorder_map, 0, inorder_map.len()-1) 
    }
}

fn build_tree(preorder: &[i32], pre_start:usize, pre_end:usize, inorder: &HashMap<i32,usize>, in_start:usize, in_end:usize) -> Option<Rc<RefCell<TreeNode>>> {
    if pre_start >= preorder.len() || pre_start > pre_end{
        return None;
    }

    let mut root = TreeNode::new(preorder[pre_start]);
    let i = *inorder.get(&preorder[pre_start]).unwrap();
    //println!("root:{} ({},{}) => {:?}",i, pre_start, pre_end, &preorder[pre_start..=pre_end]);
    
    let in_left_len = i-in_start;
    let in_right_len = in_end - i;
    root.left = build_tree(preorder, pre_start+1, pre_start+in_left_len, inorder, in_start, i-1);
    root.right = build_tree(preorder, pre_start+in_left_len+1, pre_start+in_left_len+in_right_len, inorder, i+1, in_end);
    Some(Rc::new(RefCell::new(root)))
}