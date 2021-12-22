impl Solution {
    pub fn find_anagrams(s: String, p: String) -> Vec<i32> {
        let m = s.len();
        let n = p.len();
        if n > m{
            return Vec::new();
        }
        
        let mut counts_p = [0; 26];
        for i in p.bytes().map(|c| (c - b'a') as usize) {
            counts_p[i] += 1;
        }
        let done = counts_p.iter().filter(|&&ch| ch > 0).count();

        let mut ans = Vec::new();
        let mut window_counts = [0; 26];
        let mut collected = 0;
        let s = s.as_bytes();
        for i in 0..n{
            let c = (s[i] - b'a') as usize;
            if counts_p[c] > 0 {
                window_counts[c] += 1;
                if counts_p[c] == window_counts[c] {
                    collected += 1;
                }
            }
        }
        for i in 0..m-n{
            if collected == done{
                ans.push(i as i32);
            }
            
            let c = (s[i+n] - b'a') as usize;
            if counts_p[c] > 0 {
                window_counts[c] += 1;
                if counts_p[c] == window_counts[c] {
                    collected += 1;
                }
            }
            
            let c = (s[i] - b'a') as usize;
            if counts_p[c] > 0 {
                if counts_p[c] == window_counts[c] {
                    collected -= 1;
                }
                window_counts[c] -= 1;
            }
        }
        if collected == done{
            ans.push((m-n) as i32);
        }
        ans
    }
}
