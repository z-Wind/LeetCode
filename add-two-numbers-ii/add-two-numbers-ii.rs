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
    pub fn add_two_numbers(mut l1: Option<Box<ListNode>>, mut l2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut v1 = Vec::new();
        while let Some(node) = l1{
            v1.push(node.val);
            l1 = node.next;
        }
        
        let mut v2 = Vec::new();
        while let Some(node) = l2{
            v2.push(node.val);
            l2 = node.next;
        }
        // println!("{:?}", v1);
        // println!("{:?}", v2);
        
        let mut head = None;
        let mut carry = 0;
        loop{
            match (v1.pop(), v2.pop()){
                (Some(x1), Some(x2)) => {
                    let sum = carry + x1 + x2;
                    carry = sum/10;
                    let mut node = Box::new(ListNode::new(sum%10));
                    node.next = head;
                    head = Some(node);
                },
                (Some(x), None) | (None, Some(x)) => {
                    let sum = carry + x;
                    carry = sum/10;
                    let mut node = Box::new(ListNode::new(sum%10));
                    node.next = head;
                    head = Some(node);
                },
                (None, None) => {
                    if carry > 0{
                        let mut node = Box::new(ListNode::new(carry));
                        node.next = head;
                        head = Some(node);
                    }
                    break;
                },
            }
        }
            
        head
    }
}