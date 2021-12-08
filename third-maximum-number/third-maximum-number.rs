impl Solution {
    pub fn third_max(nums: Vec<i32>) -> i32 {
        // 1st 2nd 3rd
        let mut max_vals = vec![None;3];
        for num in nums{
            for i in 0..3{
                if max_vals[i].is_none(){
                    max_vals[i] = Some(num);
                    break;
                } else if num > max_vals[i].unwrap(){
                    for j in (i+1..3).rev(){
                        max_vals[j] = max_vals[j-1];
                    }
                    max_vals[i] = Some(num);
                    break;
                } else if num == max_vals[i].unwrap(){
                    break;
                }
            }
            // println!("{:?}", max_vals);
        }
        // println!("{:?}", max_vals);
        match max_vals[2]{
            Some(x) => x,
            None => max_vals[0].unwrap()
        }
    }
}