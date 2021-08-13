impl Solution {
    pub fn first_missing_positive(nums: Vec<i32>) -> i32 {
        let mut v:Vec<i32> = nums.into_iter().filter(|x|x.is_positive()).collect();
        v.sort_unstable();
        v.dedup();
        //println!("{:?}",v);
        for (i,&x) in v.iter().enumerate(){
            let pos = i as i32 + 1;
            if pos != x{
                return pos;
            }
        }
        return v.len() as i32 +1;
    }
}