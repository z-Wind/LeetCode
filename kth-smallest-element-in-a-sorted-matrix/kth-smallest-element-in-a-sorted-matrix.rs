use std::collections::BinaryHeap;
use std::cmp::Reverse;
impl Solution {
    pub fn kth_smallest(matrix: Vec<Vec<i32>>, k: i32) -> i32 {
        let n = matrix.len();
        let mut heap = BinaryHeap::new();
        for i in 0..n{
            heap.push(Reverse((matrix[i][0], (i,1))));    
        }
        let mut ans = 0;
        for _ in 0..k{
            if let Some(Reverse((x, (i,j)))) = heap.pop(){
                // println!("{}", x);
                if j < n{
                    heap.push(Reverse((matrix[i][j], (i,j+1))));
                }
                ans = x;
            }
        }
        ans
    }
}