// https://leetcode.com/problems/relative-ranks/discuss/98468/Easy-Java-Solution-Sorting.

impl Solution {
    pub fn find_relative_ranks(score: Vec<i32>) -> Vec<String> {
        let n = score.len();
        let mut index: Vec<usize> = (0..n).collect();
        index.sort_unstable_by_key(|&i| -score[i]);

        let mut result = vec![String::new(); n];

        for i in 0..n {
            result[index[i]] = match i + 1 {
                1 => String::from("Gold Medal"),
                2 => String::from("Silver Medal"),
                3 => String::from("Bronze Medal"),
                x => x.to_string(),
            }
        }

        return result;
    }
}
