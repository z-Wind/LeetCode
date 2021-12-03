// https://leetcode.com/problems/queue-reconstruction-by-height/discuss/89345/Easy-concept-with-PythonC%2B%2BJava-Solution
//
// 1. Pick out tallest group of people and sort them in a subarray (S). Since there's no other groups of people taller than them, therefore each guy's index will be just as same as his k value.
// 2. For 2nd tallest group (and the rest), insert each one of them into (S) by k value. So on and so forth.
// E.g.
// input: [[7,0], [4,4], [7,1], [5,0], [6,1], [5,2]]
// subarray after step 1: [[7,0], [7,1]]
// subarray after step 2: [[7,0], [6,1], [7,1]]

use std::collections::{BTreeMap,BTreeSet};
impl Solution {
    pub fn reconstruct_queue(mut people: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        // obtain everyone's info
        // key=height, value=k-value, index in original array
        let mut peopledct: BTreeMap<i32, BTreeSet<(usize, usize)>> = BTreeMap::new();
        let mut res = Vec::new();

        people.iter().enumerate().for_each(|(i, p)| {
            peopledct.entry(p[0]).or_default().insert((p[1] as usize, i));
        });

        // sort from the tallest group
        for set in peopledct.values().rev() {
            for &(i, j) in set.iter() {
                res.insert(i, people[j].clone());
            }
        }

        return res;
    }
}
