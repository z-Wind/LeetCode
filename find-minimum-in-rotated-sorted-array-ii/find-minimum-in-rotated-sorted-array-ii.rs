// https://leetcode.com/problems/find-minimum-in-rotated-sorted-array-ii/discuss/48808/My-pretty-simple-code-to-solve-it/281823
//
// nums[start] vs nums[mid]
// case   nums[start]   ?   nums[mid]   ?   nums[end]    example      min_location
// 1                    <               <              [1 2 3 3 4]      left
// 2                    <               =              [1 2 3 3 3]      left
// 3                    <               >              [3 4 5 1 2]      right
// 4                    =               <              [1 1 1 2 3]      left
// 5                    =               =              [3 3 3 2 3]      right
// 6                    =               >              [3 3 3 1 2]      right
// 7                    >               <              [4 2 3 3 4]      left
// 8                    >               =              [4 2 3 3 3]      left
// 9                    >               >              [5 4 3 2 1] => impossible
//
// nums[end] vs nums[mid]
// case   nums[start]   ?   nums[mid]   ?   nums[end]    example      min_location
// 1                    <               <              [1 2 3 3 4]      left
// 4                    =               <              [1 1 1 2 3]      left
// 7                    >               <              [4 2 3 3 4]      left
// 2                    <               =              [1 2 3 3 3]      left
// 5                    =               =              [3 3 3 2 3]      right
// 8                    >               =              [4 2 3 3 3]      left
// 3                    <               >              [3 4 5 1 2]      right
// 6                    =               >              [3 3 3 1 2]      right
// 9                    >               >              [5 4 3 2 1] => impossible
//
impl Solution {
    pub fn find_min(nums: Vec<i32>) -> i32 {
        let mut start = 0;
        let mut end = nums.len()-1;
        let mut mid = 0;
        while start < end {
            mid = (start + end) / 2;
            
            if (nums[mid] < nums[end]) {
                end = mid;
            }
            else if (nums[mid] > nums[end]) {
                start = mid + 1;
            }
            else { // if nums[mid] == nums[end]
                end-=1;
            }
        }
        nums[start]
    }
}