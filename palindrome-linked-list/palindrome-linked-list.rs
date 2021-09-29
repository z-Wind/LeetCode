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
    pub fn is_palindrome(head: Option<Box<ListNode>>) -> bool {
        let mut v = Vec::new();
        let mut h = &head;
        while h.is_some(){
            v.push(h.as_ref().unwrap().val);
            h = &h.as_ref().unwrap().next;
        }
        
        let mut left = 0;
        let mut right = v.len()-1;
        while left < right{
            if v[left] != v[right]{
                return false;
            }
            left+=1;
            right-=1;
        }
        true
    }
}