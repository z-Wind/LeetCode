impl Solution {
    pub fn find_target_sum_ways(nums: Vec<i32>, target: i32) -> i32 {
        let mut count = 0;
        find_target_sum_ways(&nums, target, vec![0], &mut count);
        count
    }
}

fn find_target_sum_ways(nums: &[i32], target: i32, sums:Vec<i32>, count:&mut i32) {
    // println!("{:?}: {:?}", nums, sums);
    if nums.is_empty() {
        for sum in sums {
            if sum == target{
                *count += 1;
            }
        }
        return
    }
    let mut temp = Vec::with_capacity(sums.len()*2);
    for sum in sums {
        temp.push(sum + nums[0]);
        temp.push(sum - nums[0]);
    }
    
    
    find_target_sum_ways(&nums[1..], target, temp, count);
}