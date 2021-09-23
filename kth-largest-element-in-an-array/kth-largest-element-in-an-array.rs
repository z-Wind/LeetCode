impl Solution {
    pub fn find_kth_largest(mut nums: Vec<i32>, k: i32) -> i32 {
        let k = nums.len() - k as usize;
        // println!("k:{}",k);
        let mut left = 0;
        let mut right = nums.len()-1;
        while left < right{
            // println!("{},{}:{:?}",left, right, nums);
            let i = partition(&mut nums, left, right);
            // println!("{} => {}:{:?}",i,nums[i], nums);
            if i < k{
                left = i + 1;
            } else if i > k {
                right = i - 1;
            } else {
                break;
            }
        }
        nums[k]
    }
}

fn partition(nums: &mut Vec<i32>, mut left:usize, mut right:usize) -> usize{
    let pivot = left;
    let mut i = left + 1;
    let mut j = right;
    while true{
        while i<right && nums[i] <= nums[pivot] {i+=1}
        while j>left && nums[j] > nums[pivot] {j-=1}
        if i >= j{
            break;
        }
        nums.swap(i,j);
    }
    nums.swap(pivot,j);
    j
}