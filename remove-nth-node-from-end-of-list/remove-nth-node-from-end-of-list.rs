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
fn remove_nth_from_end(head: &mut Box<ListNode>, n: i32) -> i32 {
    let mut i:i32;
    if head.next.is_none(){
        i = 1;
    } else {
        i = remove_nth_from_end(head.next.as_mut().unwrap(), n);
        i += 1;
    }
    
    //println!("n:{}, i:{}", n,i);
    if n+1 == i{
        head.next = head.next.take().unwrap().next;
        //println!("{:?}",head);
    }
    
    return i;
}
impl Solution {
    pub fn remove_nth_from_end(mut head: Option<Box<ListNode>>, n: i32) -> Option<Box<ListNode>> {
        if n == remove_nth_from_end(head.as_mut().unwrap(), n){
            head = head.unwrap().next;
        }
        
        head
    }
}