impl Solution {
    pub fn search_range(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut i:i32 = -1;
        let mut j:i32 = -1;
        if nums.len() == 0{
            return vec![i,j];
        }
        
        let mut start = 0;
        let mut end = nums.len()-1;
        let mut middle = 1;
        while end as i32 >= start as i32{
            //println!("i,{},{}",start,end);
            middle = (start + end)/2;
            if nums[middle] == target{
                i = middle as i32;
                end = middle-1;
            } else if nums[middle] > target{
                end = middle-1;
            } else if nums[middle] < target{
                start = middle+1;
            }
        }
        
        let mut start = 0;
        let mut end = nums.len()-1;
        let mut middle = 1;
        while end as i32 >= start as i32{
            //println!("j,{},{}",start,end);
            middle = (start + end)/2;
            if nums[middle] == target{
                j = middle as i32;
                start = middle+1;
            } else if nums[middle] > target{
                end = middle-1;
            } else if nums[middle] < target{
                start = middle+1;
            }
        }
        
        vec![i,j]
    }
}
