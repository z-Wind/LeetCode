impl Solution {
    pub fn find_median_sorted_arrays(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
        let (mut i,mut j) = (0,0);
        let center:f32 = (nums1.len() as f32 + nums2.len() as f32 -1.0)/2.0;
        let (i1,i2) = (center.floor() as usize, center.ceil() as usize);
        //println!("center:{}, i1:{}, i2:{}", center, i1, i2);
        
        let mut index = i+j;
        let (mut median1, mut median2) = (0,0);
        while i < nums1.len() || j < nums2.len() {
            //println!("index:{}, i:{}, j:{}", index, i, j);
            //println!("nums1[i]:{}, nums2[j]:{}", nums1[i], nums2[j]);
            if j == nums2.len() || (i < nums1.len() && nums1[i] < nums2[j]){
                if index == i1{
                    median1 = nums1[i]
                }  
                if index == i2{
                    median2 = nums1[i]
                }
                i+=1;
            } else {
                if index == i1{
                    median1 = nums2[j]
                }  
                if index == i2{
                    median2 = nums2[j]
                }
                j+=1;
            }
            
            index = i+j;
            if index > i2{
                break
            }
        }
        //println!("median1:{}, median2:{}", median1, median2);
        (median1 as f64 + median2 as f64) /2.0
    }
}