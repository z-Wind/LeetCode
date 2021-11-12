impl Solution {
    pub fn k_smallest_pairs(nums1: Vec<i32>, nums2: Vec<i32>, k: i32) -> Vec<Vec<i32>> {
        let mut ans: Vec<Vec<i32>> = Vec::new();
        let mut iters:Vec<(i32,_)> = Vec::new();
        for num in nums1 {
            iters.push((num, nums2.iter().peekable()));
        }

        for _ in 0..k {
            let mut ele = vec![i32::MAX>>1, i32::MAX>>1];
            let mut min_i = 0;
            for i in 0..iters.len() {
                if iters[i].1.peek().is_some() && iters[i].0 + **iters[i].1.peek().unwrap() < ele[0] + ele[1] {
                    min_i = i;
                    ele[0] = iters[i].0;
                    ele[1] = **iters[i].1.peek().unwrap();
                }
            }
            if ele[0] == i32::MAX>>1{
                break;
            }
            ans.push(ele);
            iters[min_i].1.next();
        }
        
        ans
    }
}
