// https://leetcode.com/problems/heaters/discuss/95886/Short-and-Clean-Java-Binary-Search-Solution

impl Solution {
    pub fn find_radius(houses: Vec<i32>, mut heaters: Vec<i32>) -> i32 {
        let n = heaters.len();
        heaters.sort_unstable();

        let mut radius = 0;
        for house in houses {
            let r = match heaters.binary_search(&house) {
                Ok(_) => 0,
                Err(i) => match i {
                    0 => heaters[i] - house,
                    _ if i == n => house - heaters[i - 1],
                    _ => (heaters[i] - house).min(house - heaters[i - 1]),
                },
            };

            radius = radius.max(r);
        }

        radius
    }
}