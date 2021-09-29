impl Solution {
    pub fn product_except_self(mut nums: Vec<i32>) -> Vec<i32> {
        let n = nums.len();
        let mut left_prod = vec![nums[0];n];
        for i in (1..n){
            left_prod[i] = left_prod[i-1] * nums[i];
        }
        let mut right_prod = vec![nums[n-1];n];
        for i in (0..n-1).rev(){
            right_prod[i] = right_prod[i+1] * nums[i];
        }
        // println!("left :{:?}", left_prod);
        // println!("right:{:?}", right_prod);
        
        nums[0] = right_prod[1];
        nums[n-1] = left_prod[n-2];
        for i in (1..n-1){
            nums[i] = left_prod[i-1] * right_prod[i+1];
        }
        nums
    }
}