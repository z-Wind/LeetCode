impl Solution {
    pub fn find_median_sorted_arrays(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
        let (mut i,mut j) = (0,0);
        let center:f32 = (nums1.len() as f32 + nums2.len() as f32 -1.0)/2.0;
        let (i1,i2) = (center.floor() as usize, center.ceil() as usize);
        
        let (mut median1, mut median2) = (0,0);
        let mut temp;
        let mut index = i+j;
        while index <= i2 {
            if j >= nums2.len() || (i < nums1.len() && nums1[i] < nums2[j]){
                temp = nums1[i];
                i+=1;
            } else {
                temp = nums2[j];
                j+=1;
            }
            
            if index == i1{
                median1 = temp;
            }  
            if index == i2{
                median2 = temp;
            }
            index = i+j;
        }
        (median1 as f64 + median2 as f64) / 2.0
    }
}