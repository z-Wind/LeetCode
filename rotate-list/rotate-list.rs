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
    pub fn rotate_right(mut head: Option<Box<ListNode>>, k: i32) -> Option<Box<ListNode>> {
        if head.is_none(){
            return None;
        } else if k == 0{
            return head;
        }
        
        let mut len = 1;
        let mut temp:&Box<ListNode> = head.as_ref().unwrap();
        while let Some(ref node) = temp.next{
            len += 1;
            temp = node;
        }
        //println!("{}",len);
        if len == 1{
            return head;
        }
        
        let k = k % len;
        if k == 0{
            return head;
        }
        let shift = len - k -1;
        let mut temp:&mut Box<ListNode> = head.as_mut().unwrap();
        for _ in (0..shift){
            if let Some(ref mut node) = temp.next{
                temp = node;
            }
        }
        //println!("shift:{} temp:{:?}", shift, temp);
        let mut ans = temp.next.take();
        
        temp = ans.as_mut().unwrap();
        while let Some(ref mut node) = temp.next{
            temp = node;
        }
        temp.next = head;
        
        ans
    }
}
