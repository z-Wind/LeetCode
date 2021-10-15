// https://leetcode.com/problems/count-of-smaller-numbers-after-self/discuss/445769/merge-sort-CLEAR-simple-EXPLANATION-with-EXAMPLES-O(n-lg-n)

// Wrapper class for each and every value of the input array,
// to store the original index position of each value, before we merge sort the array
#[derive(Debug, Clone, Copy)]
struct ArrayValWithOrigIdx {
    val: i32,
    originalIdx: usize,
}

impl ArrayValWithOrigIdx {
    fn new(val:i32, originalIdx:usize) -> Self{
        Self{
            val,
            originalIdx,
        }
    }
}

fn mergeSortAndCount(nums:&mut Vec<ArrayValWithOrigIdx>, start:usize, end:usize, result:&mut Vec<i32>) {
    if start >= end {
        return;
    }

    let mid = (start + end) / 2;
    mergeSortAndCount(nums, start, mid, result);
    mergeSortAndCount(nums, mid + 1, end, result);

    // left subarray start...mid
    // right subarray mid+1...end
    let mut leftPos = start;
    let mut rightPos = mid + 1;
    let mut merged:Vec<ArrayValWithOrigIdx> = Vec::new();
    let mut numElemsRightArrayLessThanLeftArray = 0;
    while leftPos < mid + 1 && rightPos <= end {
        if nums[leftPos].val > nums[rightPos].val {
            // this code block is exactly what the problem is asking us for:
            // a number from the right side of the original input array, is smaller
            // than a number from the left side
            //
            // within this code block,
            // nums[rightPos] is smaller than the start of the left sub-array.
            // Since left sub-array is already sorted,
            // nums[rightPos] must also be smaller than the entire remaining left sub-array
            numElemsRightArrayLessThanLeftArray+=1;

            // continue with normal merge sort, merge
            merged.push(nums[rightPos].clone());
            rightPos+=1;
        } else {
            // a number from left side of array, is smaller than a number from
            // right side of array
            result[nums[leftPos].originalIdx] += numElemsRightArrayLessThanLeftArray;

            // Continue with normal merge sort
            merged.push(nums[leftPos].clone());
            leftPos+=1;
        }
    }

    // part of normal merge sort, if either left or right sub-array is not empty,
    // move all remaining elements into merged result
    while leftPos < mid + 1 {
        result[nums[leftPos].originalIdx] += numElemsRightArrayLessThanLeftArray;

        merged.push(nums[leftPos].clone());
        leftPos+=1;
    }
    while (rightPos <= end) {
        merged.push(nums[rightPos].clone());
        rightPos+=1;
    }

    // part of normal merge sort
    // copy back merged result into array
    let mut pos = start;
    for m in merged {
        nums[pos] = m.clone();
        pos+=1;
    }
}

impl Solution {
    pub fn count_smaller(nums: Vec<i32>) -> Vec<i32> {
        if nums.is_empty() {
            return vec![];   
        }
         
        let n = nums.len();
        let mut result = vec![0;n];
        
        let mut newNums:Vec<ArrayValWithOrigIdx> = Vec::with_capacity(n);
        for i in 0..n{
            newNums.push(ArrayValWithOrigIdx::new(nums[i], i));
        }

        mergeSortAndCount(&mut newNums, 0, n - 1, &mut result);

        result
    }
}