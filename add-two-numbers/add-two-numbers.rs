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
     fn add_two_numbers_with_carry(carry:i32, l1: Option<Box<ListNode>>, l2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        match (carry, l1, l2){
            (0,None,None) => None,
            (c,None,None) => Some(Box::new(ListNode::new(c))),
            (0,None,Some(y)) => Some(y),
            (c,None,Some(y)) => {
                let sum = c + y.val;
                let val = sum % 10;
                let carry:i32 = sum/10;
                
                let next = Solution::add_two_numbers_with_carry(carry,None, y.next);
                let mut l = ListNode::new(val);
                l.next = next;
                Some(Box::new(l))
            },
            (0,Some(x),None) => Some(x),
            (c,Some(x),None) => {
                let sum = c + x.val;
                let val = sum % 10;
                let carry:i32 = sum/10;
                
                let next = Solution::add_two_numbers_with_carry(carry, x.next, None);
                let mut l = ListNode::new(val);
                l.next = next;
                Some(Box::new(l))
            },
            (c,Some(x),Some(y)) => {
                let sum = c + x.val + y.val;
                let val = sum % 10;
                let carry:i32 = sum/10;
                
                let next = Solution::add_two_numbers_with_carry(carry, x.next, y.next);
                let mut l = ListNode::new(val);
                l.next = next;
                return Some(Box::new(l));
            },
        }
    }
    pub fn add_two_numbers(l1: Option<Box<ListNode>>, l2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        Solution::add_two_numbers_with_carry(0, l1, l2)
    }
}