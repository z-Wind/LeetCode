impl Solution {
    pub fn missing_number(mut nums: Vec<i32>) -> i32 {
        let n = nums.len();
        for i in (0..n){
            let mut num = nums[i].abs() as usize;
            if num == n + 1{
                num = 0;
            }
            if num < n{
                nums[num] = -nums[num];
                if nums[num] == 0{
                    nums[num] = -(n as i32) - 1;
                }
            }
        }
        // println!("{:?}", nums);
        match nums.iter().position(|&x| x>=0) {
            Some(x) => x as i32,
            None => n as i32,
        }
    }
}