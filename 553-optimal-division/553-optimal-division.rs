// https://leetcode.com/problems/optimal-division/discuss/101687/Easy-to-understand-simple-O(n)-solution-with-explanation
// X1/X2/X3/../Xn = (X1/X2) * Y
// maximum Y because of Xn >= 2
// X1/(X2/X3/../Xn) = (X1 *X3 *..*Xn)/X2

impl Solution {
    pub fn optimal_division(nums: Vec<i32>) -> String {
        let n = nums.len();
        match n {
            0 => panic!("impossible"),
            1 => nums[0].to_string(),
            2 => {
                format!("{}/{}", nums[0], nums[1])
            }
            _ => {
                let mut result = format!("{}/({}", nums[0], nums[1]);
                
                for i in 2..n {
                    result.push('/');
                    result.push_str(&nums[i].to_string());
                }
                
                result.push(')');
                
                result
            },
        }    
    }
}