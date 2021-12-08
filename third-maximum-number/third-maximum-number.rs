use std::cmp::Ordering;
impl Solution {
    pub fn third_max(nums: Vec<i32>) -> i32 {
        // 1st 2nd 3rd
        let mut max_vals = vec![None;3];
        for num in nums{
            let num = Some(num);
            for i in 0..3{
                match num.cmp(&max_vals[i]){
                    Ordering::Less => (),
                    Ordering::Equal => break,
                    Ordering::Greater => {
                        for j in (i+1..3).rev(){
                            max_vals[j] = max_vals[j-1];
                        }
                        max_vals[i] = num;
                        break;
                    },
                }
            }
            // println!("{:?}", max_vals);
        }
        // println!("{:?}", max_vals);
        max_vals[2].unwrap_or(max_vals[0].unwrap())
    }
}