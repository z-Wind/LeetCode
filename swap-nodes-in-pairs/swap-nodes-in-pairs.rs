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
    pub fn swap_pairs(mut head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        if head.is_none(){
            return None;
        }

        let next = head.as_mut().unwrap().next.take();
        match (head, next){
            (None, _) => return None,
            (head,None) => return head,
            (mut left,mut right) => {
                let next = Solution::swap_pairs(right.as_mut().unwrap().next.take());
                left.as_mut().unwrap().next = next;            
                right.as_mut().unwrap().next = left;

                return right;
            },
        }
    }
}