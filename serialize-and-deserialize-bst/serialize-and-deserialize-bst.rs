// https://github.com/z-Wind/LeetCode/blob/main/serialize-and-deserialize-binary-tree/serialize-and-deserialize-binary-tree.rs

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
struct Codec {
	
}

/** 
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl Codec {
    fn new() -> Self {
        Self{}
    }

    fn serialize(&mut self, root: Option<Rc<RefCell<TreeNode>>>) -> String {
        if root.is_none(){
            return String::from("");
        }
        let left = root.as_ref().unwrap().borrow_mut().left.take();
        let right = root.as_ref().unwrap().borrow_mut().right.take();
        format!("{},{},{}", 
            root.as_ref().unwrap().borrow().val.to_string(),
            self.serialize(left),
            self.serialize(right),
        )
    }
	
    fn deserialize(&mut self, data: String) -> Option<Rc<RefCell<TreeNode>>> {
        // println!("{}",data);
        if data.is_empty(){
            return None;
        }
        let mut nodes = data.split(',');
        self.build_tree(&mut nodes)
    }
    
    fn build_tree<'a, I>(&self, nodes:&mut I) -> Option<Rc<RefCell<TreeNode>>> 
    where
        I: Iterator<Item = &'a str>,
    {
        let val = nodes.next().unwrap();
        if val == "" {
            return None;
        } else {
            let mut node = TreeNode::new(val.parse().unwrap());
            node.left = self.build_tree(nodes);
            node.right = self.build_tree(nodes);
            
            return Some(Rc::new(RefCell::new(node)));
        }
    }
}

/**
 * Your Codec object will be instantiated and called as such:
 * let obj = Codec::new();
 * let data: String = obj.serialize(strs);
 * let ans: Option<Rc<RefCell<TreeNode>>> = obj.deserialize(data);
 */