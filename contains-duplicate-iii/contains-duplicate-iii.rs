impl Solution {
    pub fn contains_nearby_almost_duplicate(nums: Vec<i32>, k: i32, t: i32) -> bool {
        let w_size = k as usize + 1;
        if nums.len() < w_size{
            for i in (0..nums.len()){
                for j in (i+1..nums.len()){
                    if sub_abs_compare(nums[i], nums[j], t){
                        return true;
                    }
                }
            }
            return false;
        }
        
        for (i,w) in nums.windows(w_size).enumerate(){
            // println!("{}: {:?}",i,w);
            if i == 0{
                for j in (0..w_size){
                    for k in (j+1..w_size){
                        if sub_abs_compare(w[j], w[k], t){
                            return true;
                        }
                    }
                }
            } else {
                for j in (0..w_size-1){
                    if sub_abs_compare(w[j], w[w_size-1], t){
                        return true;
                    }
                }
            }
        }
        false
    }
}

fn sub_abs_compare(a:i32, b:i32, t:i32) -> bool{
    match a.overflowing_sub(b){
        (_,true) => false,
        (x,false) => {
            if (x < 0 && x >= -t) ||
               (x >= 0 && x <= t) {
                   return true;
            } 
            false
        }
    }
}