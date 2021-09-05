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
use std::cell::RefCell;
use std::rc::Rc;
use std::cmp::max;

#[derive(Debug)]
struct Path{
    seperate_max:i32, // only_left, only_right, left_to_right, only_root
    link:i32, // link_left, link_right, with_only_root
}

impl Path{
    fn new(val:i32) -> Self{
        Path{
            seperate_max:val,
            link:val,
        }
    }
    fn max(&self) -> i32{
        self.seperate_max.max(self.link)
    }
    fn update_seperate_max(&mut self, val:i32){
         self.seperate_max = self.seperate_max.max(val);
    }
}

impl Solution {
    pub fn max_path_sum(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let path = max_path_sum(root);
        // println!("root: {:?}", path);
        path.max()
    }
}

fn max_path_sum(root: Option<Rc<RefCell<TreeNode>>>) -> Path{    
    let val = root.as_ref().unwrap().borrow().val;
    let left = root.as_ref().unwrap().borrow().left.clone();
    let right = root.as_ref().unwrap().borrow().right.clone();
    
    match (left, right){
        (None, None) => {
            Path::new(val)
        },
        (Some(x), None) | (None, Some(x)) => {
            let mut path = max_path_sum(Some(Rc::clone(&x)));
            // println!("{}: {:?}", val, path);
            
            // only_left, only_right
            path.update_seperate_max(x.borrow().val); 
            path.update_seperate_max(path.link); 
            // only_root
            path.update_seperate_max(val) ;
            
            // link_left, link_right, with_only_root
            path.link = max(val, path.link+val);
            
            path
        }
        (Some(l), Some(r)) => {
            let mut left_path = max_path_sum(Some(l));
            let mut right_path = max_path_sum(Some(r));
            // println!("{}: left: {:?}", val, left_path);
            // println!("{}:right: {:?}", val, right_path);
            
            // only_left, only_right
            left_path.update_seperate_max(right_path.seperate_max); 
            left_path.update_seperate_max(left_path.link); 
            left_path.update_seperate_max(right_path.link); 
            // only_root
            left_path.update_seperate_max(val);
            // left_to_right
            left_path.update_seperate_max(left_path.link + val + right_path.link); 
            
            // link_left, link_right, with_only_root
            let mut link = max(left_path.link+val,right_path.link+val);
            left_path.link = max(link, val);
            
            left_path
        },
    }    
}
