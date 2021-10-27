// https://leetcode.com/problems/verify-preorder-serialization-of-a-binary-tree/discuss/78551/7-lines-Easy-Java-Solution/188285

impl Solution {
    pub fn is_valid_serialization(preorder: String) -> bool {
        let mut edges = 1; // 1 for root
        for node in preorder.split(',') {
            edges-=1; // consume one edge
            if edges < 0 { return false; } // to prevent the case: [#,](https://leetcode.com/problems/powx-n) a, ...
            if node != "#"{
                edges += 2; // generate 2 edges
            }
        }
        edges == 0
    }
}