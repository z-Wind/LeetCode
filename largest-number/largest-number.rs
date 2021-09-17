use std::cmp::Ordering;
impl Solution {
    pub fn largest_number(nums: Vec<i32>) -> String {
        let mut nums = nums.iter()
                        .map(|num| num.to_string())
                        .collect::<Vec<String>>();
        nums.sort_unstable_by(|a, b| cmp(a, b).reverse());
        if nums[0] == "0"{
            return String::from("0");
        }
        nums.join("").trim_start_matches('0').to_string()
    }
}

fn cmp(a:&str, b:&str) -> Ordering{
    (a.to_owned()+b).cmp(&(b.to_owned()+a))
}