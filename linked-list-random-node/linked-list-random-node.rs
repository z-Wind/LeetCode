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

// https://zhuanlan.zhihu.com/p/29178293
// Reservoir Sampling
use rand::{thread_rng, Rng};

struct Solution {
    head:Option<Box<ListNode>>,
}


/** 
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl Solution {

    fn new(head: Option<Box<ListNode>>) -> Self {        
        Self{
            head
        }
    }
    
    fn get_random(&self) -> i32 {
        let mut rng = thread_rng();
        let k = 1;
        let mut vals = vec![0;k];
        let mut i = 1;
        
        let mut h = &self.head;
        while let Some(node) = h{
            let sel = rng.gen_range(0, i);
            if sel < k {
                vals[sel] = node.val;
            }
            h = &node.next;
            i+=1;
        }
        
        vals[0]
    }
}

/**
 * Your Solution object will be instantiated and called as such:
 * let obj = Solution::new(head);
 * let ret_1: i32 = obj.get_random();
 */