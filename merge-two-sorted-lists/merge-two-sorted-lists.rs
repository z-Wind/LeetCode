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
    pub fn merge_two_lists(l1: Option<Box<ListNode>>, l2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        match (l1, l2){
            (None, None) => None,
            (None, l) | (l, None) => l,
            (mut l1, mut l2) =>{
                let mut dummy = ListNode::new(0);
                let mut p = &mut dummy;
                while l1.is_some() && l2.is_some(){
                    let v1 = l1.as_ref().unwrap().val ;
                    let v2 = l2.as_ref().unwrap().val;
                    if v1 < v2 {
                        p.next = Some(Box::new(ListNode::new(v1)));
                        l1 = l1.unwrap().next.take();
                    } else {
                        p.next = Some(Box::new(ListNode::new(v2)));
                        l2 = l2.unwrap().next.take();
                    }
                    p = p.next.as_mut().unwrap();
                }
                while l1.is_some(){
                    p.next = Some(Box::new(ListNode::new(l1.as_ref().unwrap().val)));
                    l1 = l1.unwrap().next.take();
                    p = p.next.as_mut().unwrap();
                }
                while l2.is_some(){
                    p.next = Some(Box::new(ListNode::new(l2.as_ref().unwrap().val)));
                    l2 = l2.unwrap().next.take();
                    p = p.next.as_mut().unwrap();
                }
                dummy.next
            },
        }
    }
}