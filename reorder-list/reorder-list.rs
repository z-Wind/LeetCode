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
enum Order{
    Back,
    Front,
}
use std::collections::VecDeque;
impl Solution {
    pub fn reorder_list(mut head: &mut Option<Box<ListNode>>) {
        let mut nodes:VecDeque<Option<Box<ListNode>>> = VecDeque::new();
        
        let mut h = head.as_mut().unwrap().next.take();
        let mut next = None;
        while h.is_some(){
            next = h.as_mut().unwrap().next.take();
            nodes.push_back(h);
            h = next;
        }
        // println!("{:?}", nodes);
        let mut order = Order::Back;
        while !nodes.is_empty(){
            let node:Option<Box<ListNode>>;
            match order{
                Order::Back => {
                    node=nodes.pop_back().unwrap();
                    order=Order::Front;
                },
                Order::Front => {
                    node=nodes.pop_front().unwrap();
                    order=Order::Back;
                },
            }
            head.as_mut().unwrap().next = node;
            head = &mut head.as_mut().unwrap().next;
        }
    }
}