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
    pub fn reverse_list(head:Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        if head.is_none(){
            return None;
        }
        let mut v:Vec<Option<Box<ListNode>>> = Vec::new();
        let mut h = head;
        while h.is_some(){
            let mut next = h.as_mut().unwrap().next.take();
            v.push(h);
            h = next;
        }
        
        h = Some(Box::new(ListNode::new(0)));
        let mut p = &mut h;
        while let Some(node) = v.pop(){
            p.as_mut().unwrap().next = node;
            p = &mut p.as_mut().unwrap().next;
        }
        h.as_mut().unwrap().next.take()
    }
}