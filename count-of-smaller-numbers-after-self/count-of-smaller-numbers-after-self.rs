impl Solution {
    pub fn count_smaller(nums: Vec<i32>) -> Vec<i32> {
        let n = nums.len();
        let mut nums_sorted = Vec::with_capacity(n);
        
        let mut ans:Vec<i32> = vec![0;n];
        for i in (0..n).rev(){
            let count = match nums_sorted.binary_search(&nums[i]){
                Err(idx) => {
                    nums_sorted.insert(idx, nums[i]);
                    idx
                },
                Ok(mut idx) => {
                    nums_sorted.insert(idx, nums[i]);
                    while idx != 0 && nums_sorted[idx-1] == nums_sorted[idx]{
                        idx -= 1;
                    }  
                    idx
                },
            };
            
            ans[i] = count as i32;
            // println!("{} => {}: {:?}",nums[i], count, nums_sorted);
        }
        
        ans
    }
}