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
    pub fn path_sum(root: Option<Rc<RefCell<TreeNode>>>, target_sum: i32) -> Vec<Vec<i32>> {
        let mut ans:Vec<Vec<i32>> = Vec::new();
        let mut cur:Vec<i32> = Vec::new();
        path_sum(&mut ans, &mut cur, root, target_sum);
        ans
    }
}

pub fn path_sum(ans:&mut Vec<Vec<i32>>, cur:&mut Vec<i32>, root: Option<Rc<RefCell<TreeNode>>>, mut target_sum: i32) {
    if root.is_none(){
        return;
    }
    
    let val = root.as_ref().unwrap().borrow().val;
    cur.push(val);
    target_sum -= val;       

    let left = root.as_ref().unwrap().borrow().left.clone();
    let right = root.as_ref().unwrap().borrow().right.clone();

    match (left,right){
        (None,None) => {
            if target_sum == 0{
                ans.push(cur.clone());
            }
        },
        (Some(x),None) | (None,Some(x)) => path_sum(ans, cur, Some(x), target_sum),
        (Some(l),Some(r)) => {
            path_sum(ans, cur, Some(l), target_sum);
            path_sum(ans, cur, Some(r), target_sum);
        },
    }
    cur.pop();
}
