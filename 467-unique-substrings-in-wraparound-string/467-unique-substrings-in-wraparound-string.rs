impl Solution {
    pub fn find_substring_in_wrapround_string(p: String) -> i32 {
        let mut counts:[i32;26] = [0;26];
        let p = p.as_bytes();
        let n = p.len();
        let mut i = 0;
        while i < n {
            let mut count = 1;
            let c = (p[i] - b'a') as usize;
            counts[c] = counts[c].max(count);
            let mut pre = p[i];
            let mut end = i;
            for j in i+1..n{
                if p[j] == pre + 1 || (pre == b'z' && p[j] == b'a'){
                    count += 1;
                    end += 1;
                    let c = (p[j] - b'a') as usize;
                    counts[c] = counts[c].max(count);
                } else {
                    break;
                }
                pre = p[j];
            }
            i = end + 1;
        }
        // println!("{:?}", counts);
        counts.iter().sum()
    }
}