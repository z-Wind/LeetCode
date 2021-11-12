impl Solution {
    pub fn k_smallest_pairs(nums1: Vec<i32>, nums2: Vec<i32>, k: i32) -> Vec<Vec<i32>> {
        let k = k as usize;
        let n1 = nums1.len();
        let n2 = nums2.len();
        let mut ans: Vec<Vec<i32>> = Vec::with_capacity(k);
        let mut idx = vec![0;n1];
        for _ in 0..k {
            let mut ele:Vec<i32> = vec![i32::MAX>>1, i32::MAX>>1];
            let mut min_i=0;
            for i in 0..n1 {
                if idx[i] < n2 && nums1[i] + nums2[idx[i]] < ele[0] + ele[1]{
                    ele[0] = nums1[i];
                    ele[1] = nums2[idx[i]];
                    min_i = i;
                }
            }
            if ele[0] == i32::MAX>>1{
                break;
            }
            ans.push(ele);
            idx[min_i] += 1;
        }
        
        ans
    }
}
