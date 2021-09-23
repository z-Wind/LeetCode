use std::collections::BinaryHeap;
use std::cmp::Reverse;
impl Solution {
    pub fn find_kth_largest(nums: Vec<i32>, k: i32) -> i32 {
        let k = k as usize;
        let mut heap = BinaryHeap::with_capacity(k);
        for num in nums{
            if heap.len() < k{
                heap.push(Reverse(num));
            } else if num > heap.peek().unwrap().0{
                heap.pop();
                heap.push(Reverse(num));
            }
        }
        heap.peek().unwrap().0
    }
}