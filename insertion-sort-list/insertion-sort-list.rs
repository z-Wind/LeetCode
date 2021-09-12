// Definition for singly-linked list.
// #[derive(PartialEq, Eq, Clone, Debug)]
// pub struct ListNode {
//   pub val: i32,
//   pub next: Option<Box<ListNode>>
// }
// 
// impl ListNode {
//   #[inline]
//   fn new(val: i32) -> Self {
//     ListNode {
//       next: None,
//       val
//     }
//   }
// }
use std::mem::swap;
impl Solution {
    pub fn insertion_sort_list(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut new_h = Some(Box::new(ListNode::new(i32::MIN)));
        
        let mut h = head;
        while h.is_some(){
            let next = h.as_mut().unwrap().next.take();
            let i_val = h.as_ref().unwrap().val;
            let mut node:&mut Option<Box<ListNode>> = &mut new_h;
            let mut next_node:Option<Box<ListNode>>;
            loop {
                if node.as_mut().unwrap().next.is_none(){
                    node.as_mut().unwrap().next = h;
                    break;
                }
                
                next_node = node.as_mut().unwrap().next.take();
                let val = node.as_ref().unwrap().val;
                let next_val = next_node.as_ref().unwrap().val;
                if val <= i_val && i_val <= next_val{
                    h.as_mut().unwrap().next = next_node;        
                    node.as_mut().unwrap().next = h;
                    break;
                }
                node.as_mut().unwrap().next = next_node;
                node = &mut node.as_mut().unwrap().next;
            }
            
            h = next;
        }
        
        new_h.as_mut().unwrap().next.take()
    }
}