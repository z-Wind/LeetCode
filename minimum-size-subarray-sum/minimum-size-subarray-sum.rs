impl Solution {
    pub fn min_sub_array_len(target: i32, nums: Vec<i32>) -> i32 {
        let mut left:usize = 0;
        let mut right:usize = 0;
        let mut sum = 0;
        let mut min_len = i32::MAX;
        while left < nums.len(){
            if right < nums.len() && sum < target{
                sum+=nums[right];
                right+=1;
            } else if sum >= target{
                let len = (right-left) as i32;
                min_len = min_len.min(len);
                sum-=nums[left];
                left+=1;
            } else {
                break;
            }
            // println!("{:?} {} => {}",&nums[left..right],sum,min_len);
        }
        if min_len == i32::MAX{
            return 0;
        }
        min_len
    }
}