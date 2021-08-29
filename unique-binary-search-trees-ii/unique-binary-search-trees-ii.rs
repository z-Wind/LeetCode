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
    pub fn generate_trees(n: i32) -> Vec<Option<Rc<RefCell<TreeNode>>>> {
        let mut nums:Vec<i32> = (1..=n).collect();
        let mut ans:Vec<Option<Rc<RefCell<TreeNode>>>> = Vec::new();
        
        for i in (0..nums.len()){
            for root in create_tree(nums[i], &nums[..i], &nums[i+1..]){
                ans.push(root);
            }
        }
        ans
    }
}

fn clone_node(mut root:&Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>>{
    let mut node = root.clone();
    node
}

fn create_tree(root:i32, left:&[i32] ,right:&[i32]) -> Vec<Option<Rc<RefCell<TreeNode>>>>{
    //println!("{:?} <- {} -> {:?}", left, root, right);
    if left.is_empty() && right.is_empty(){
        return vec![Some(Rc::new(RefCell::new(TreeNode::new(root))))];
    }
    
    let mut lefts:Vec<Option<Rc<RefCell<TreeNode>>>> = Vec::new();
    for i in (0..left.len()){
        lefts.append(&mut create_tree(left[i], &left[..i], &left[i+1..]));
    }
    // println!("{}: lefts: {:?}", root, lefts);
    
    let mut rights:Vec<Option<Rc<RefCell<TreeNode>>>> = Vec::new();
    for i in (0..right.len()){
        rights.append(&mut create_tree(right[i], &right[..i], &right[i+1..]));
    }
    // println!("{}: rights: {:?}", root, rights);
    
    let mut roots:Vec<Option<Rc<RefCell<TreeNode>>>> = Vec::new();
    if lefts.is_empty(){
        for right in rights.iter(){
            let mut root = TreeNode::new(root);
            root.right = clone_node(right);
            roots.push(Some(Rc::new(RefCell::new(root))));
        }
    } else if rights.is_empty(){
        for left in lefts.iter(){
            let mut root = TreeNode::new(root);
            root.left = clone_node(left);
            roots.push(Some(Rc::new(RefCell::new(root))));
        }
    } else {
        for left in lefts.iter(){
            for right in rights.iter(){
                let mut root = TreeNode::new(root);
                root.left = clone_node(left);
                root.right = clone_node(right);
                roots.push(Some(Rc::new(RefCell::new(root))));
            }
        }
    }
    
    //println!("roots: {:?}", roots);
    
    roots
}