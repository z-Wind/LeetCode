impl Solution {
    pub fn min_window(s: String, t: String) -> String {
        let mut map_t= [0 as isize; (b'z' - b'A' + 1) as usize];
        for b in t.bytes() {
            map_t[(b - b'A') as usize] += 1;
        }
        let sb: Vec<(usize, u8)> = s
            .match_indices(|c| map_t[(c as u8 - b'A') as usize]>0)
            .map(|(i, s)| (i, s.bytes().next().unwrap()))
            .collect();

        //println!("{:?}", sb);

        let (mut l, mut r) = (0, 0);
        let mut ans = (usize::MAX, None, None);
        let mut collected = 0;
        let done = map_t.iter().filter(|&&ch| ch > 0).count();
        let mut window_count = [0 as isize; (b'z' - b'A' + 1) as usize];
        while r < sb.len() {
            let c = (sb[r].1 - b'A') as usize;
            if map_t[c] > 0{
                window_count[c] += 1;
                if map_t[c] == window_count[c]{
                    collected += 1;
                }
                
            }
            while l <= r && collected == done {
                let c =(sb[l].1 - b'A') as usize;
                if map_t[c]>0{
                    let range = sb[r].0 - sb[l].0 + 1;
                    if range < ans.0 {
                        ans = (range, Some(sb[l].0), Some(sb[r].0));
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