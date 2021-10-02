// https://leetcode.com/problems/h-index/discuss/70768/Java-bucket-sort-O(n)-solution-with-detail-explanation

impl Solution {
    pub fn h_index(citations: Vec<i32>) -> i32 {
        let n = citations.len();
        let mut h_count = vec![0;n+1];
        for c in citations{
            let i = n.min(c as usize);
            h_count[i] += 1;
        }
        // println!("{:?}", h_count);
        let mut count = 0;
        for i in (0..=n).rev(){
            count += h_count[i];
            if count >= i{
                return i as i32;
            }
        }
        0
    }
}