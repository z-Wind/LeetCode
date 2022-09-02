impl Solution {
    pub fn array_pair_sum(mut nums: Vec<i32>) -> i32 {
        nums.sort_unstable();
        
        let mut ans = 0;
        for w in nums.chunks_exact(2) {
            ans += w[0].min(w[1]);
        }

        ans
    }
}
