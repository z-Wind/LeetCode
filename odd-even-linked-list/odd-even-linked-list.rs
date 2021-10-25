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
    pub fn odd_even_list(mut head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        if head.is_none() || head.as_ref().unwrap().next.is_none(){
            return head
        }
        
        let mut odd = &mut head;
        let mut next = odd.as_mut().unwrap().next.take();
        let mut even = &mut next;
        
        while even.is_some() && even.as_ref().unwrap().next.is_some(){
            odd.as_mut().unwrap().next = even.as_mut().unwrap().next.take();
            odd = &mut odd.as_mut().unwrap().next;
            even.as_mut().unwrap().next = odd.as_mut().unwrap().next.take();
            even = &mut even.as_mut().unwrap().next;
        }
        odd.as_mut().unwrap().next = next;
        
        head
    }
}