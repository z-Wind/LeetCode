// https://leetcode.com/problems/minimum-moves-to-equal-array-elements/discuss/93817/It-is-a-math-question
// sum + (n-1) * move = n * final_val
// final_val = min_num + move 
// the minum number will be incremented in every move. So, if the final number is x, it would be minNum + moves;
// sum + (n-1) * move = n * (min_num + move)
// sum + n * move - move = n * min_num + n * move
// move = sum - n * min_num
impl Solution {
    pub fn min_moves(mut nums: Vec<i32>) -> i32 {
        let n = nums.len() as i32;
        let mut min = nums[0];
        let mut sum = 0;
        for &num in nums.iter(){
            sum += num;
            min = min.min(num);
        }
        
        sum - n*min
    }
}