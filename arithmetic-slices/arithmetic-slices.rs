// if n >= 3
// n-3+1
// n-4+1
// n-n+1
// sum: (n-3+1)*(n+1) - sum(3..=n)
//    = (n-2)(n+1) - (sum(1..=n) - (1+2))
//    = (n-2)(n+1) - ((n+1)*n/2 - 3)
//    = (n+1)/2 * [ 2*(n-2) - n] + 3
//    = (n+1)/2 * (n-4) + 3
//    = (n+1)(n-4)/2 + 3

impl Solution {
    pub fn number_of_arithmetic_slices(nums: Vec<i32>) -> i32 {
        let len = nums.len();
        if len < 3{
            return 0;
        }
        
        let mut count:i32 = 0;
        
        let mut i = 0;
        while i < len-2{
            let mut n:i32 = 2;
            let diff = nums[i+1] - nums[i];
            for j in i+1..len-1{
                if nums[j+1] - nums[j] == diff{
                    n+=1;
                } else {
                    break;
                }
            }
            
            if n >= 3{
                count += ((n+1)*(n-4)/2+3);    
            }
            i += (n as usize - 1);
        }
        count
    }
}