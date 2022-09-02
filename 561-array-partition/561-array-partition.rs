// https://leetcode.com/problems/array-partition/discuss/102180/Java-O(n)-beats-100
// Counting Sort

const K: usize = 10000;

impl Solution {
    pub fn array_pair_sum(nums: Vec<i32>) -> i32 {
        // Store the frequency of each element
        let mut elementToCount = vec![0; 2 * K + 1];
        for element in nums {
            // Add K to element to offset negative values
            elementToCount[element as usize + K] += 1;
        }

        // Initialize sum to zero
        let mut maxSum: i32 = 0;
        let mut isEvenIndex = true;
        for element in 0..=2 * K {
            while elementToCount[element] > 0 {
                // Add element if it is at even position
                maxSum += if isEvenIndex { (element - K) as i32 } else { 0 };
                // Flip the value (one to zero or zero to one)
                isEvenIndex = !isEvenIndex;
                // Decrement the frequency count
                elementToCount[element] -= 1;
            }
        }

        maxSum
    }
}
