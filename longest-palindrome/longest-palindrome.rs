impl Solution {
    pub fn longest_palindrome(s: String) -> i32 {
        let mut counts:Vec<i32> = vec![0;26*2];
        for c in s.bytes(){
            let idx:usize = if c.is_ascii_lowercase(){
                (c-b'a') as usize
            } else {
                (c-b'A') as usize + 26
            };
            counts[idx] += 1;
        }
        let mut ans = 0;
        for count in counts{
            // count/2 * 2
            ans += (count & -2)
        }
        if ans == s.len() as i32{
            return ans;
        }
        ans + 1
    }
}