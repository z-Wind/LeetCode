// https://leetcode.com/problems/wiggle-sort-ii/discuss/77677/O(n)%2BO(1)-after-median-Virtual-Indexing
//
// find mid
// three-way partitioning to 1 3 5 7 0 2 4 6
// so larger at 1 3 5, mid at 7 0, smaller at 2 4 6

use rand::thread_rng;
use rand::seq::SliceRandom;

impl Solution {
    pub fn wiggle_sort(nums: &mut Vec<i32>) {
        let n = nums.len();
    
        // Find a median.
        let mid = nth_element(nums, n/2);
        // println!("mid:{}", mid);
        
        // Index-rewiring.
        // 0 1 2 3 4 5
        // 1 3 5 0 2 4
        let A = |i| {
            (1 + 2 * i) % (n | 1)
        };

        // 3-way-partition-to-wiggly in O(n) time with O(1) space.
        let mut i = 0;
        let mut j = 0;
        let mut k = n - 1;
        while j <= k {
            // println!("i:{}=>{}, j:{}=>{}, k:{}=>{}, {:?}", i, A(i), j, A(j), k, A(k), nums);
            if nums[A(j)] > mid{
                nums.swap(A(i), A(j));
                i+=1;
                j+=1;
            } else if nums[A(j)] < mid{
                nums.swap(A(j), A(k));
                k-=1;
            }
            else{
                j+=1;
            }
        }
    }
}

fn nth_element(nums: &mut Vec<i32>, k: usize) -> i32 {
    nums.shuffle(&mut thread_rng());
    let mut left = 0;
    let mut right = nums.len()-1;
    while left < right{
        let i = partition(nums, left, right);
        if i < k{
            left = i + 1;
        } else if i > k {
            right = i - 1;
        } else {
            break;
        }
    }
    nums[k]
}

fn partition(nums: &mut Vec<i32>, mut left:usize, mut right:usize) -> usize{
    let pivot = left;
    let mut i = left + 1;
    let mut j = right;
    loop{
        while i<right && nums[i] <= nums[pivot] {i+=1}
        while j>left && nums[j] > nums[pivot] {j-=1}
        if i >= j{
            break;
        }
        nums.swap(i,j);
    }
    nums.swap(pivot,j);
    j
}
