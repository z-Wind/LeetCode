use std::collections::HashMap;

impl Solution {
    pub fn intersect(mut nums1: Vec<i32>, mut nums2: Vec<i32>) -> Vec<i32> {
        let mut map1:HashMap<i32,usize> = HashMap::new();
        if nums1.len() > nums2.len(){
            std::mem::swap(&mut nums1, &mut nums2);
        }
        for num in nums1{
            *map1.entry(num).or_insert(0) += 1;
        }
        
        let mut ans:Vec<i32> = Vec::new();
        for num in nums2{
            match map1.get_mut(&num){
                Some(val) if *val > 0 => {
                    ans.push(num);
                    *val-=1;
                },
                _ => (),
            }
        }
        
        
        ans
    }
}