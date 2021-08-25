impl Solution {
    pub fn merge(nums1: &mut Vec<i32>, m: i32, nums2: &mut Vec<i32>, n: i32) {
        for i in (0..n as usize){
            nums1[i+m as usize] = nums2[i];
        }
        nums1.sort()
    }
}