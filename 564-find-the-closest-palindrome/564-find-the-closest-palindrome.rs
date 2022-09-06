use std::cmp::Ordering;

impl Solution {
    pub fn nearest_palindromic(n: String) -> String {
        let num: i64 = n.parse().expect("should be number");
        let prefix = (n.len() + 1) / 2;
        let suffix = n.len() - prefix;

        let mut prefix_num: i64 = n
            .chars()
            .take(prefix)
            .collect::<String>()
            .parse()
            .expect("should be number");

        let shift_prefix = 10_i64.pow(suffix as u32);
        let equal = prefix_num * shift_prefix + reverse_num(prefix_num, 10, suffix);
        let diff_equal = (equal - num).abs();
        prefix_num -= 1;
        let less = prefix_num * shift_prefix + reverse_num(prefix_num, 10, suffix);
        let diff_less = (less - num).abs();
        prefix_num += 2;
        let more = prefix_num * shift_prefix + reverse_num(prefix_num, 10, suffix);
        let diff_more = (more - num).abs();

        let mut candidates = vec![(diff_equal, equal), (diff_less, less), (diff_more, more)];
        candidates.sort_unstable();
        // println!("{:?}", candidates);
        if candidates[0].0 != 0 {
            return candidates[0].1.to_string();
        }
        return candidates[1].1.to_string();
    }
}

fn reverse_num(mut num: i64, radix: i64, suffix: usize) -> i64 {
    let mut reversed = 0;
    let mut n = 0;
    while num != 0 {
        reversed = reversed * radix + num % radix;
        num /= radix;
        n += 1;
    }

    if n >= suffix {
        return reversed % 10_i64.pow(suffix as u32);
    }

    (10i64.pow((suffix - n) as u32) - 1) * 10i64.pow(n as u32)
        + reversed % 10_i64.pow(suffix as u32)
}
