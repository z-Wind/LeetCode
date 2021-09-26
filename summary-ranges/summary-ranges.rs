impl Solution {
    pub fn summary_ranges(nums: Vec<i32>) -> Vec<String> {        
        let mut list:Vec<String> = Vec::new();
        let mut start = 0;
        let mut end = 0;
        while end < nums.len(){
            while(end < nums.len() - 1 && nums[end] + 1 == nums[end + 1]){
                end+=1;
            }
            if(start == end){
                list.push(format!("{}",nums[start]));
            } else {
                list.push(format!("{}->{}",nums[start],nums[end]));
            }
            end+=1;
            start = end;
        }
        
        list
    }
}