/** 
 * The rand7() API is already defined for you.
 * @return a random integer in the range 1 to 7
 * fn rand7() -> i32;
 */

// https://leetcode.com/problems/implement-rand10-using-rand7/discuss/150301/Three-line-Java-solution-the-idea-can-be-generalized-to-"Implement-RandM()-Using-RandN()"/196858

impl Solution {
    pub fn rand10() -> i32 {
        loop {
            let rand = 7 * (rand7() - 1) + (rand7() - 1); // 0 to 48
            if rand < 40 {
                return rand % 10 + 1;
            }
        }
    }
}