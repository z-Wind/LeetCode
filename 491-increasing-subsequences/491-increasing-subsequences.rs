use std::collections::HashSet;
impl Solution {
    pub fn find_subsequences(mut nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut ans = HashSet::new();
        let mut temp = Vec::new();
        find_subsequences(&mut ans, &mut temp, &nums, 0);

        ans.into_iter().collect()
    }
}

fn find_subsequences(ans: &mut HashSet<Vec<i32>>, temp: &mut Vec<i32>, nums: &Vec<i32>, start: usize) {
    if ans.contains(temp){
        return;
    }
    if temp.len() >= 2 {
        ans.insert(temp.clone());
    }
    if start == nums.len() {
        return;
    }

    let x = if let Some(&x) = temp.last() {
        x
    } else {
        i32::MIN
    };
    for i in start..nums.len() {
        if x > nums[i] || (i != start && nums[i - 1] == nums[i]) {
            continue;
        }

        temp.push(nums[i]);
        find_subsequences(ans, temp, nums, i + 1);
        temp.pop();
    }
}