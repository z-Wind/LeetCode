impl Solution {
    pub fn search(nums: Vec<i32>, target: i32) -> i32 {
        search(&nums, target, 0, nums.len()-1)
    }
}

pub fn search(nums: &Vec<i32>, target: i32, start: usize, end: usize) -> i32 {
    let middle = (start + end)/2;
    if nums[middle] == target{
        return middle as i32;
    } else if nums[start] == target{
        return start as i32;
    } else if nums[end] == target{
        return end as i32;
    } else if start >= end{
        return -1;
    }
    
    
    //println!("{},{},{},{:?}",start,middle,end, &nums[start..=end]);
    
    let mut i = -1;
    if (nums[middle] > nums[start] && nums[start] < target && target < nums[middle]) ||
       (nums[middle] < nums[start] && (nums[start] < target || nums[middle] > target)){
        i = search(&nums, target, start, middle);
    } else{
        i = search(&nums, target, middle+1, end);
    }
    return i;
}