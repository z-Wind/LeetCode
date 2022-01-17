impl Solution {
    pub fn find_max_form(strs: Vec<String>, m: i32, n: i32) -> i32 {
        let mut zeros_ones: Vec<(i32, i32)> = strs
            .iter()
            .map(|s| {
                s.chars().fold((0, 0), |acc, c| {
                    if c == '0' {
                        (acc.0 + 1, acc.1)
                    } else {
                        (acc.0, acc.1 + 1)
                    }
                })
            })
            .collect();
        zeros_ones.sort_unstable();
        // println!("{:?}", zeros_ones);
        
        let mut ans = 0;
        find_max_form(&mut ans, 0, (0,0), &zeros_ones, 0, m, n);
        ans
    }
}

fn find_max_form(
    ans: &mut i32,
    level: i32,
    counts: (i32, i32),
    zeros_ones: &Vec<(i32, i32)>,
    start: usize,
    m: i32,
    n: i32,
) {
    if counts.0 > m || counts.1 > n{
        return;
    }
    *ans = (*ans).max(level);
    
    let len = zeros_ones.len();
    if start >= len {
        return;
    }
    
    // println!("{} {:?} {:?}", level, counts, &zeros_ones[start..]);

    for i in start..len {
        if i > start && zeros_ones[i] == zeros_ones[i-1]{
            continue;
        }
        let zeros = counts.0 + zeros_ones[i].0;
        let ones = counts.1 + zeros_ones[i].1;
        if zeros > m {
            break;
        }
        if ones > n {
            continue;
        }
        
        find_max_form(
            ans,
            level + 1,
            (zeros, ones),
            zeros_ones,
            i + 1,
            m,
            n,
        )
    }
}
