// https://leetcode.com/problems/wiggle-subsequence/discuss/84843/Easy-understanding-DP-solution-with-O(n)-Java-version
// up position, it means nums[i] > nums[i-1]
// down position, it means nums[i] < nums[i-1]
// equals to position, nums[i] == nums[i-1]
// 
// when up/down does not upate, could change ref num
// [3,4,3,2,2,3]
//        3  ->  4  -> 3  -> 2  -> 2  -> 3
// up   1(3)  2(4)   2(4)  2(4)  2(4)  4(3)      
// down 1(3)  1(4)   3(3)  3(2)  3(2)  3(3)

impl Solution {
    pub fn wiggle_max_length(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        if (n < 2){
            return n as i32;
        }
            
        let mut down = 1;
        let mut up = 1;
        for i in 1..n {
            if nums[i] > nums[i - 1]{
                up = down + 1;
            } else if nums[i] < nums[i - 1]{
                down = up + 1;
            }
        }
        down.max(up)
    }
}
