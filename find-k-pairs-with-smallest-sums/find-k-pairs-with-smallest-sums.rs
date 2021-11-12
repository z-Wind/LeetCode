use std::collections::BinaryHeap;

struct Ele(i32, i32);
impl Ord for Ele {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        (self.0 + self.1).cmp(&(other.0 + other.1)).reverse()
    }
}

impl PartialOrd for Ele {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Ele {
    fn eq(&self, other: &Self) -> bool {
        (self.0, self.1) == (other.0, other.1)
    }
}

impl Eq for Ele { }

impl Solution {
    pub fn k_smallest_pairs(nums1: Vec<i32>, nums2: Vec<i32>, k: i32) -> Vec<Vec<i32>> {
        let mut heap:BinaryHeap<Ele> = BinaryHeap::new();
        let mut ans: Vec<Vec<i32>> = Vec::new();
        let mut iters:Vec<(i32,_)> = Vec::new();
        for num in nums1 {
            iters.push((num, nums2.iter().peekable()));
        }
        
        let mut i = 0;
        while i < k {
            for j in 0..iters.len() {
                match iters[j].1.next(){
                    Some(&x) => heap.push(Ele(iters[j].0, x)),
                    None => (),
                }    
            }
            match heap.pop(){
                Some(ele) => {
                    ans.push(vec![ele.0, ele.1]);
                    i+=1;
                },
                None => break,
            }
        }
        
        ans
    }
}
