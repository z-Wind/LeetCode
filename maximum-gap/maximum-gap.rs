// https://leetcode.com/problems/maximum-gap/discuss/50643/bucket-sort-JAVA-solution-with-explanation-O(N)-time-and-space
impl Solution {
    pub fn maximum_gap(mut nums: Vec<i32>) -> i32 {
        if nums.len() < 2{
            return 0;
        }
        let mut num_min = i32::MAX;
        let mut num_max = i32::MIN;
        for &num in nums.iter(){
            num_min = num_min.min(num);
            num_max = num_max.max(num);
        }
        if num_min == num_max{
            return 0;
        }
        let mut avg_gap = (num_max-num_min) / (nums.len() - 1) as i32;
        // ceil
        if (num_max-num_min) % (nums.len() - 1) as i32 != 0{
            avg_gap += 1;
        }
        // println!("min:{}, max:{}, avg_gap:{}", num_min, num_max, avg_gap);
        let mut buckets_min:Vec<Option<i32>> = vec![None;nums.len()];
        let mut buckets_max:Vec<Option<i32>> = vec![None;nums.len()];
        for num in nums{
            let i = ((num-num_min)/avg_gap) as usize;
            // println!("{} => {}",num,i);
            buckets_min[i] = Some(buckets_min[i].map_or(num, |x| x.min(num)));
            buckets_max[i] = Some(buckets_max[i].map_or(num, |x| x.max(num)));
        }
        // println!("buckets_min: {:?}",buckets_min);
        // println!("buckets_max: {:?}",buckets_max);
        let mut max_gap = i32::MIN;
        let mut prev = num_min;
        for i in (0..buckets_min.len()){
            match (buckets_min[i],buckets_max[i]){
                (None,None) => continue,
                (Some(num_min), Some(num_max)) => {
                    max_gap = max_gap.max(num_min - prev);
                    prev = num_max;
                },
                _ => panic!(),
            }
        }
        max_gap
    }
}