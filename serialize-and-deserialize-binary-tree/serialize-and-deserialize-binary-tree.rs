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
            return String::new();
        }
        let left = root.as_ref().unwrap().borrow_mut().left.take();
        let right = root.as_ref().unwrap().borrow_mut().right.take();
        format!("{}({},{})", 
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
        let mut count = 0;
        let mut root_end = 0;
        let mut split = 0;
        for (i,char) in data.chars().enumerate(){
            match char{
                '(' => {
                    if count == 0{
                        root_end = i;
                    }
                    count+=1;
                },
                ')' => count-=1,
                ',' if count == 1 => {
                    split = i;
                    break;
                },
                _ => (),
            }    
        }
        let val:i32 = data[0..root_end].parse().unwrap();
        // println!("{}",val);
        let mut root = Some(Rc::new(RefCell::new(TreeNode::new(val))));
        root.as_ref().unwrap().borrow_mut().left = self.deserialize(data[root_end+1..split].to_string());
        root.as_ref().unwrap().borrow_mut().right = self.deserialize(data[split+1..data.len()-1].to_string());
        root
    }
}

/**
 * Your Codec object will be instantiated and called as such:
 * let obj = Codec::new();
 * let data: String = obj.serialize(strs);
 * let ans: Option<Rc<RefCell<TreeNode>>> = obj.deserialize(data);
 */