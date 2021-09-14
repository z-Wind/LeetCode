impl Solution {
    pub fn find_min(nums: Vec<i32>) -> i32 {
        search(&nums,0,nums.len()-1)
    }
}

pub fn search(nums: &Vec<i32>, start: usize, end: usize) -> i32 {
    if start == end{
        return nums[start]
    }
    let mid = (start + end)/2;
    
    // 1 2 3 4 5
    if (nums[end] > nums[start]){
        search(&nums, start, mid)
    } else { // 3 4 5 1 2
        if nums[mid] >= nums[start]{
            search(&nums, mid+1, end)
        } else {
            search(&nums, start, mid)
        }
    }
}