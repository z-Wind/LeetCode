// https://leetcode.com/problems/smallest-good-base/discuss/96587/Python-solution-with-detailed-mathematical-explanation-and-derivation

impl Solution {
    pub fn smallest_good_base(n: String) -> String {
        let n: u128 = n.parse().unwrap();
        let max_m = (n as f64).log2() as u32; // Refer [7]
        for m in (2..=max_m).rev() {
            let k = (n as f64).powf(1.0 / m as f64) as u128; // Refer [6]
            if (k.pow(m + 1) - 1) / (k - 1) == n {
                // Refer [3]
                return k.to_string();
            }
        }

        return (n - 1).to_string();
    }
}
