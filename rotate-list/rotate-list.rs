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
        let mut v:Vec<Box<ListNode>> = vec![];
        while let Some(mut node) = head{
            head = node.next.take();
            v.push(node);
        }
        if v.len() == 0{
            return None;
        }
        
        let k = k as usize % v.len();
        v.rotate_right(k);
        //println!("{:?}",v);
        let mut temp = ListNode::new(0);
        for mut node in v.into_iter().rev(){
            node.next = temp.next.take();
            temp.next = Some(node);
        }
        
        temp.next
    }
}