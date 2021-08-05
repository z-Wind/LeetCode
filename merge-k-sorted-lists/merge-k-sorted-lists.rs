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
    pub fn merge_k_lists(mut lists: Vec<Option<Box<ListNode>>>) -> Option<Box<ListNode>> {
        if lists.len() == 0{
            return None;
        }
        while lists.len() > 1{
            let m = merge_two_lists(lists.remove(0), lists.remove(0));
            lists.push(m);
        }
        
        lists.remove(0)
    }
}

fn merge_two_lists(l1: Option<Box<ListNode>>, l2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
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
                    p.next = l1.take();
                    l1 = p.next.as_mut().unwrap().next.take();
                } else {
                    p.next = l2.take();
                    l2 = p.next.as_mut().unwrap().next.take();
                }
                p = p.next.as_mut().unwrap();
            }
            if l1.is_some(){
                p.next = l1.take();
            }else if l2.is_some(){
                p.next = l2.take();
            }
            dummy.next
        },
    }
}