use std::cmp::min;
impl Solution {
    pub fn find_min(nums: Vec<i32>) -> i32 {
        search(&nums,0,nums.len()-1)
    }
}

fn search(nums: &Vec<i32>, start: usize, end: usize) -> i32 {
    if start == end{
        return nums[start]
    }
    let mid = (start + end)/2;

    // 1 2 2 4 5
    if (nums[end] > nums[start]){
         search(&nums, start, mid)
    // 3 4 5 1 2 or 2 1
    } else if (nums[end] < nums[start]){
        // 3 4 5
        if nums[mid] >= nums[start]{
            search(&nums, mid+1, end)
        // 3 4 1
        } else {//if nums[mid] < nums[start] {
            search(&nums, start, mid)           
        }
    } else { // 3 4 5 2 3 or 3 4 1 2 2 2 3 or 4 1 2 3 
        if nums[mid] > nums[start]{
            search(&nums, mid+1, end)
        } else if nums[mid] < nums[start] {
            search(&nums, start, mid)
        } else {
            min(search(&nums, start, mid), search(&nums, mid+1, end))   
        }
    }
}