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
    pub fn binary_tree_paths(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<String> {        
        if root.is_none(){
            return vec![];
        }
        
        let mut ans:Vec<String> = Vec::new();
        dfs(&mut ans, String::new(), root);
        ans
    }
}

fn dfs(ans:&mut Vec<String>, path:String, root: Option<Rc<RefCell<TreeNode>>>){
    let val = root.as_ref().unwrap().borrow().val;
    let left = root.as_ref().unwrap().borrow().left.clone();
    let right = root.as_ref().unwrap().borrow().right.clone();
    match (left,right){
        (None,None) => {
            ans.push(format!("{}{}",path,val));
        },
        (x,None) | (None,x) =>{
            dfs(ans,format!("{}{}->",path,val),x);          
        },
        (left, right) =>{
            dfs(ans,format!("{}{}->",path,val),left);
            dfs(ans,format!("{}{}->",path,val),right);       
        },
    }
}