impl Solution {
    pub fn product_except_self(mut nums: Vec<i32>) -> Vec<i32> {
        let n = nums.len();
        let mut left_prod = vec![1;n];
        for i in (1..n){
            left_prod[i] = left_prod[i-1] * nums[i-1];
        }
        // println!("{:?}", left_prod);
        
        let mut right_prod = 1;
        for i in (0..n).rev(){
            left_prod[i] *= right_prod;
            right_prod *= nums[i];
        }
        
        left_prod
    }
}