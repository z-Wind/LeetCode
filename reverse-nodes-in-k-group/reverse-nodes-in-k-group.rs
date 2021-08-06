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
    pub fn reverse_k_group(mut head: Option<Box<ListNode>>, k: i32) -> Option<Box<ListNode>> {
        // if head.is_none(){
        //     return None;
        // }
        let mut node = head.as_ref();
        for i in (0..k){
            if node.is_none(){
                return head;
            }
            node = node.as_ref().unwrap().next.as_ref();
        }
        
        
        let mut vec:Vec<Option<Box<ListNode>>> = vec![];
        let mut next = head.as_mut().unwrap().next.take();
        for i in (0..k-1){
            vec.push(head);
            head = next;
            next = head.as_mut().unwrap().next.take();
            if next.is_none(){
                break;
            }
        }
        
        // println!("vec:{:?}", vec);
        // println!("head:{:?}", head);
        // println!("next:{:?}", next);
        
        let mut node = head.as_mut().unwrap();
        for v in vec.into_iter().rev(){
            node.next = v;
            node = node.next.as_mut().unwrap();
        }
        
        // println!("node:{:?}",node);
        
        node.next = Solution::reverse_k_group(next, k);
        
        head
    }
}
