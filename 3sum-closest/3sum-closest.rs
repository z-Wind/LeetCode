impl Solution {
    pub fn three_sum_closest(mut nums: Vec<i32>, target: i32) -> i32 {
        let mut ans:Option<i32> = None;
        
        nums.sort();
        
        let mut i=0;
        'outer: for i in (0..nums.len()-2){
            if i>0 && nums[i-1] == nums[i]{
                continue;
            }
            
            let mut j=i+1;
            let mut k=nums.len()-1;
            let mut sum:Option<i32> = None;
            
            while j < k{
                //println!("{}:{},{}:{},{}:{}",i,nums[i],j,nums[j],k,nums[k]);
                sum = Some(nums[i] + nums[j] + nums[k]);
                if ans.is_none() || (sum.unwrap()-target).abs() < (ans.unwrap()-target).abs(){
                    ans = sum;
                }
                
                if (sum.unwrap() < target) {
                    j+=1;
                    continue;
                } else if (sum.unwrap() > target) {
                    k-=1;
                    continue;
                }
                
                ans = sum;
                break 'outer;
            }
        }
        
        ans.unwrap()
    }
}

