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
    pub fn remove_elements(mut head: Option<Box<ListNode>>, val: i32) -> Option<Box<ListNode>> {
        let mut node = Some(Box::new(ListNode::new(0)));
        node.as_mut().unwrap().next = head;
        let mut h = &mut node;
        while h.is_some(){
            // println!("{:?}",h);
            let mut next = h.as_mut().unwrap().next.take();
            if next.is_some() && val == next.as_ref().unwrap().val{
                h.as_mut().unwrap().next = next.as_mut().unwrap().next.take();
            } else {
                h.as_mut().unwrap().next = next;
                h = &mut h.as_mut().unwrap().next;
            }
        }
        
        node.as_mut().unwrap().next.take()
    }
}