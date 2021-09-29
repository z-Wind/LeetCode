impl Solution {
    pub fn product_except_self(mut nums: Vec<i32>) -> Vec<i32> {
        let mut product = 1;
        let mut zeros = 0;
        for num in nums.iter(){
            if *num == 0{
                zeros += 1;
            } else {
                if zeros >= 2{
                    product = 0;
                    break;
                }
                product *= num;
            }
        }
        if zeros == 1{
            for num in nums.iter_mut(){
                if *num == 0{
                    *num = product;
                } else {
                    *num = 0;
                }
            }
        } else if zeros > 1{
            for num in nums.iter_mut(){
                *num = 0;
            }
        } else {
            for num in nums.iter_mut(){
                *num = product/(*num);
            }
        }
        nums
    }
}