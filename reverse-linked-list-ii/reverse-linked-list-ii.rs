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
use std::collections::VecDeque;
impl Solution {
    pub fn reverse_between(head: Option<Box<ListNode>>, left: i32, right: i32) -> Option<Box<ListNode>> {
        let mut h = head;
        let mut stack:Vec<Box<ListNode>> = vec![];
        let mut deque:VecDeque<Box<ListNode>> = VecDeque::new();
        let mut i = 0;
        while let Some(mut node) = h{
            h = node.next.take();
            //println!("{}", node.val);
            i+=1;
            if i < left{
                stack.push(node);
            } else {
                deque.push_back(node);
            }
            if i == right{
                break;
            }            
        }
        //println!("{:?}, {:?}", stack, deque);
        
        while !deque.is_empty(){
            let mut node = deque.pop_front().unwrap();
            node.next = h;
            h = Some(node);
        }
        while !stack.is_empty(){
            let mut node = stack.pop().unwrap();
            node.next = h;
            h = Some(node);
        }
        h
    }
}