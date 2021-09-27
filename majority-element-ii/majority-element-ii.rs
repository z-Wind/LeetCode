// https://leetcode.com/problems/majority-element-ii/discuss/63520/Boyer-Moore-Majority-Vote-algorithm-and-my-elaboration
// https://leetcode.com/problems/majority-element-ii/discuss/63537/My-understanding-of-Boyer-Moore-Majority-Vote
// Boyer-Moore Majority Vote Algorithm

impl Solution {
    pub fn majority_element(nums: Vec<i32>) -> Vec<i32> {
        if nums.is_empty(){
            return vec![];
        }
        let mut candicate1 = 0;
        let mut count1 = 0;
        let mut candicate2 = 1;
        let mut count2 = 0;
        for &num in nums.iter(){
            match num{
                x if x == candicate1 => count1+=1,
                x if x == candicate2 => count2+=1,
                _ => {
                    match (count1,count2){
                        (0,_) => {
                            candicate1 = num;
                            count1 = 1;
                        },
                        (_,0) => {
                            candicate2 = num;
                            count2 = 1;
                        },
                        (_,_) => {
                            count1 -= 1;
                            count2 -= 1;
                        }
                    }
                }
            }
        }
        // println!("{}:{}, {}:{}",candicate1,count1,candicate2,count2);
        let mut result = vec![];
        count1 = 0;
        count2 = 0;
        let n = nums.len()/3;
        for num in nums{
            if num == candicate1{
                count1 += 1
            } else if num == candicate2{
                count2 += 1
            } 
        }
        if count1 > n{
            result.push(candicate1)
        }
        if count2 > n{
            result.push(candicate2)
        }
        result
        
    }
}