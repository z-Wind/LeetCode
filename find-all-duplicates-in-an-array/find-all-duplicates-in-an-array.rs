impl Solution {
    pub fn find_duplicates(mut nums: Vec<i32>) -> Vec<i32> {        
        let n = nums.len();
        let mut ans = vec![];
        for i in 0..n{
            if nums[i] <= 0{
                continue;
            }
            // println!("i:{}", i);
            
            let mut num = nums[i];
            let mut idx = num as usize -1;
            if idx == i{
                nums[i] = 0;
                continue;
            } else {
                nums[i] = -2;   
            }
            loop{
                // println!("{}:{} {:?}", num, nums[idx], nums);
                if nums[idx] == 0{
                    ans.push(num);
                    nums[idx] = -1;
                    break;
                } else {
                    if nums[idx] == -2{
                        nums[idx] = 0;
                        break;
                    }
                    num = nums[idx];
                    nums[idx] = 0;
                    idx = num as usize -1;
                }
            }
        }
        // println!("{:?}", nums);
        ans
    }
}