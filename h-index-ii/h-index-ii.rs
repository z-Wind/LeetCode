// https://leetcode.com/problems/h-index-ii/discuss/71063/Standard-binary-search

impl Solution {
    pub fn h_index(citations: Vec<i32>) -> i32 {
        let n = citations.len();
        let mut left = 0;
        let mut right = citations.len()-1;
        while left <= right{
            let mid = (left+right)/2;
            // println!("{},{},{}",left,mid,right);
            if citations[mid] as usize >= n - mid{
                right = mid - 1;
                if right >= n { break; }
            } else {
                left = mid + 1;
            }
        }
        (n - left) as i32
    }
}