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
        let mut v:Vec<Option<Box<ListNode>>> = Vec::new();
        
        let mut h = head;
        while h.is_some(){
            let val = h.as_ref().unwrap().val;
            let next = h.as_mut().unwrap().next.take();            
            let mut i = 0;
            while i<v.len() && v[i].as_ref().unwrap().val < val{
                i+=1;
            }
            v.insert(i,h);
            
            h = next;
        }
        
        let mut new_h = Some(Box::new(ListNode::new(0)));
        let mut h = &mut new_h;
        for node in v{
            h.as_mut().unwrap().next = node;
            h = &mut h.as_mut().unwrap().next;
        }
        new_h.as_mut().unwrap().next.take()
    }
}