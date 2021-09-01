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
    pub fn build_tree(inorder: Vec<i32>, mut postorder: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
        let mut inorder_map:HashMap<i32,usize> = HashMap::new();
        for (i,val) in inorder.into_iter().enumerate(){
            inorder_map.insert(val,i);
        }
        postorder.reverse();
        build_tree(&postorder[..], 0, postorder.len()-1, &inorder_map, 0, inorder_map.len()-1) 
    }
}

fn build_tree(postorder: &[i32], post_start:usize, post_end:usize, inorder: &HashMap<i32,usize>, in_start:usize, in_end:usize) -> Option<Rc<RefCell<TreeNode>>> {
    if post_start >= postorder.len() || post_start > post_end{
        return None;
    }

    let mut root = TreeNode::new(postorder[post_start]);
    let i = *inorder.get(&postorder[post_start]).unwrap();
    // println!("root:{} ({},{}) => {:?}",i, post_start, post_end, &postorder[post_start..=post_end]);
    
    let in_left_len = i-in_start;
    let in_right_len = in_end - i;
    // println!("in_start:{}, in_end:{}, in_left_len: {}, in_right_len:{}", in_start, in_end, in_left_len, in_right_len);
    root.right = build_tree(postorder, post_start+1, post_start+in_right_len, inorder, i+1, in_end);
    root.left = build_tree(postorder, post_start+in_right_len+1, post_start+in_left_len+in_right_len, inorder, in_start, i-1);
    Some(Rc::new(RefCell::new(root)))
}