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
use std::mem;

impl Solution {
    pub fn reverse_k_group(mut head: Option<Box<ListNode>>, k: i32) -> Option<Box<ListNode>> {
        // if head.is_none(){
        //     return None;
        // }
        let mut tail = &mut head;
        for _ in (0..k){
            if tail.is_none(){
                return head;
            }
            tail = &mut tail.as_mut().unwrap().next;
        }
        let tail = tail.take();
        
        let mut next = Solution::reverse_k_group(tail, k);
        
        // reverse k nodes
        for _ in (0..k-1){
            mem::swap(&mut head.as_mut().unwrap().next, &mut next);
            mem::swap(&mut head, &mut next);
        }
        mem::swap(&mut head.as_mut().unwrap().next, &mut next);      
        
        head
    }
}
