impl Solution {
    pub fn max_product(nums: Vec<i32>) -> i32 {
        if nums.len() == 1{
            return nums[0];
        }
        
        let mut max_product = i32::MIN;
        for i in (0..nums.len()){
            let mut product = nums[i];
            if product>max_product{
                max_product = product;
            }
            for j in (i+1..nums.len()){
                product*=nums[j];
                if product>max_product{
                    max_product = product;
                }
            }
        }
        
        max_product
    }
}