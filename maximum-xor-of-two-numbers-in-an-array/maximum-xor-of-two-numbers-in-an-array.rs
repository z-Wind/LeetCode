impl Solution {
    pub fn find_maximum_xor(mut nums: Vec<i32>) -> i32 {
        let n = nums.len();
        if n == 1{
            return 0;
        } else if n == 2{
            return nums[0] ^ nums[1];
        }
        nums.sort_unstable_by(|a,b| b.cmp(a));
        nums.dedup();
        let zeros:Vec<u32> = nums.iter().map(|x| x.leading_zeros()).collect();
        
        let mut ans:i32 = 0;
        let mut zero = 32;
        for i in 0..nums.len(){
            if zeros[i] > zero{
                break;
            }
            for j in i+1..nums.len(){
                // println!("{:010b}, {:010b}", nums[i], nums[j]);
                let val = nums[i] ^ nums[j];
                if val > ans{
                    ans = val;
                    zero = val.leading_zeros();
                }
            }
        }
        ans
    }
}