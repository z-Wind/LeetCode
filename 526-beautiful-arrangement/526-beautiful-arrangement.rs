impl Solution {
    pub fn count_arrangement(n: i32) -> i32 {
        let mut nums = (1..=n).collect();
        count_arrangement(nums, 1)
    }
}

fn count_arrangement(mut nums: Vec<i32>, idx: i32) -> i32 {
    if nums.is_empty() {
        return 1;
    }
    
    // println!("{}:{:?}", idx, nums);
    let mut counts = 0;
    for i in (0..nums.len()).rev() {
        if nums[i] % idx == 0 || idx % nums[i] == 0 {
            let num = nums.swap_remove(i);
            counts += count_arrangement(nums.clone(), idx+1);
            nums.push(num);
        }
    }
    // println!("{}:{:?} => {}", idx, nums, counts);
    counts
}