// https://leetcode.com/problems/ugly-number-ii/discuss/69364/My-16ms-C%2B%2B-DP-solution-with-short-explanation
// nums[1] = min( nums[0]x2, nums[0]x3, nums[0]x5)
// nums[2] = min( nums[1]x2, nums[0]x3, nums[0]x5)
// And so on. Be careful about the cases such as 6, in which we need to forward both pointers of 2 and 3.

impl Solution {
    pub fn nth_ugly_number(n: i32) -> i32 {
        if n == 1{
            return 1;    
        }
        let n = n as usize;
        let mut nums = vec![1];
        
        let mut i = 0;
        let mut j = 0;
        let mut k = 0;
        while nums.len() < n{
            let x = nums[i]*2;
            let y = nums[j]*3;
            let z = nums[k]*5;
            let num = x.min(y).min(z);
            nums.push(num);
            if x == num{
                i+=1;
            }
            if y == num{
                j+=1;
            }
            if z == num{
                k+=1;
            }
        }
        // println!("{:?}", nums);
        nums[n-1]
    }
}