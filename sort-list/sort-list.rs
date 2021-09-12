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
    pub fn sort_list(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut v:Vec<Option<Box<ListNode>>> = Vec::new();
        
        let mut h = head;
        while h.is_some(){
            let next = h.as_mut().unwrap().next.take();            
            v.push(h);            
            h = next;
        }
        v.sort_unstable_by(|a, b| a.as_ref().unwrap().val.cmp( &b.as_ref().unwrap().val));
        let mut new_h = Some(Box::new(ListNode::new(0)));
        let mut h = &mut new_h;
        for node in v{
            h.as_mut().unwrap().next = node;
            h = &mut h.as_mut().unwrap().next;
        }
        new_h.as_mut().unwrap().next.take()
    }
}