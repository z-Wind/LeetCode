// https://leetcode.com/problems/valid-perfect-square/discuss/83874/A-square-number-is-1%2B3%2B5%2B7%2B...-JAVA-code

impl Solution {
    pub fn is_perfect_square(num: i32) -> bool {
        let num = num as i64;
        let mut low = 1;
        let mut high = num;
        while low <= high {
            let mid = low + ((high-low)>>1);
            // println!("{},{} => {}",low, high, mid);
            match (mid * mid).cmp(&num) {
                std::cmp::Ordering::Equal => return true,
                std::cmp::Ordering::Less => low = mid + 1,
                std::cmp::Ordering::Greater => high = mid - 1,
            }
        }
        false
    }
}
