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
impl Solution {
    pub fn partition(head: Option<Box<ListNode>>, x: i32) -> Option<Box<ListNode>> {
        if head.is_none(){
            return None;
        }
        
        let mut h = head;
        let mut left = Box::new(ListNode::new(0));
        let mut left_p = &mut left;
        let mut right = Box::new(ListNode::new(0));
        let mut right_p = &mut right;
        while let Some(mut node) = h{
            println!("{:?}", node);
            h = node.next.take();
            
            if node.val < x{
                left_p.next = Some(node);
                left_p = left_p.next.as_mut().unwrap();
            } else {
                right_p.next = Some(node);
                right_p = right_p.next.as_mut().unwrap();
            }
            
        }
        left_p.next = right.next;
        left.next
    }
}