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
    pub fn delete_duplicates(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut head_new = ListNode::new(0);
        let mut next_new = &mut head_new;
        
        let mut i = 0;
        let mut pre:i32=i32::MAX;
        let mut val:i32=i32::MAX;
        let mut next = head.as_ref();
        while let Some(ref node) = next{
            if i == 0{
                val = node.val;
            } else { 
                pre = val;
                val = node.val;
                //println!("{},{}", pre, val);

                if pre != val {
                    next_new.next = Some(Box::new(ListNode::new(pre)));
                    next_new = next_new.next.as_mut().unwrap();
                }
            }
            
            next = node.next.as_ref();
            i+=1;
        }
        if val != i32::MAX{
            next_new.next = Some(Box::new(ListNode::new(val)));
        }

        head_new.next
    }
}