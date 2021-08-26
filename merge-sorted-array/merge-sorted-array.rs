use std::mem::swap;
impl Solution {
    pub fn merge(nums1: &mut Vec<i32>, m: i32, nums2: &mut Vec<i32>, n: i32) {
        if nums2.is_empty(){
            return;
        }
        let m = m as usize;
        let n = n as usize;
        
        let mut j = 0;
        for i in (0..m+n){
            if i >= m{
                nums1[i] = nums2[j];
                j+=1;
            } else if nums1[i] <= nums2[j]{
            } else {
                swap(&mut nums1[i], &mut nums2[j]);
                let k = j;
                while j+1<n && nums2[j+1] < nums2[j]{
                    nums2.swap(j,j+1);
                    j+=1
                }
                j = k;
            }
            //println!("nums1:{:?}, nums2:{:?}",nums1,nums2);
        }
    }
}