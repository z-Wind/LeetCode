impl Solution {
    pub fn min_window(s: String, t: String) -> String {
        let mut map_t= [0 as isize; (b'z' - b'A' + 1) as usize];
        for b in t.bytes() {
            map_t[(b - b'A') as usize] += 1;
        }
        let sb = s.as_bytes();

        //println!("{:?}", map_t);

        let (mut l, mut r) = (0, 0);
        let mut ans = (usize::MAX, None, None);
        let mut collected = 0;
        let done = map_t.iter().filter(|&&ch| ch > 0).count();
        let mut window_count = [0 as isize; (b'z' - b'A' + 1) as usize];
        while r < sb.len() {
            let c = (sb[r] - b'A') as usize;
            if map_t[c] > 0{
                window_count[c] += 1;
                if map_t[c] == window_count[c]{
                    collected += 1;
                }
                
            }
            while l <= r && collected == done {
                let c =(sb[l] - b'A') as usize;
                if map_t[c]>0{
                    let range = r - l + 1;
                    if range < ans.0 {
                        ans = (range, Some(l), Some(r));
                    }
                    window_count[c] -= 1;
                    if map_t[c] > window_count[c]{
                        collected -= 1;
                    }
                    //println!("{},{} => {:?}",l,r,ans);
                }
                l += 1;
            }
            r += 1;
        }

        match ans{
            (_,None,None) => String::from(""),
            (_,Some(start),Some(end)) => s[start..=end].to_string(),
            _ => panic!(),
        }
    }
}