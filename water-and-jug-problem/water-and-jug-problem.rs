// https://leetcode.com/problems/water-and-jug-problem/discuss/83715/Math-solution-Java-solution
// Bézout's lemma
// Bézout's identity (also called Bézout's lemma) is a theorem in the elementary theory of numbers:
// let a and b be nonzero integers and let d be their greatest common divisor. Then there exist integers x and y such that ax+by=d
// In addition, the greatest common divisor d is the smallest positive integer that can be written as ax + by
// every integer of the form ax + by is a multiple of the greatest common divisor d.

impl Solution {
    pub fn can_measure_water(jug1_capacity: i32, jug2_capacity: i32, target_capacity: i32) -> bool {
        jug1_capacity == target_capacity
            || jug2_capacity == target_capacity
            || jug1_capacity + jug2_capacity == target_capacity
            || (jug1_capacity + jug2_capacity > target_capacity
                && (target_capacity % gcd(jug1_capacity, jug2_capacity)) == 0)
    }
}

fn gcd(a: i32, b: i32) -> i32 {
    if b == 0 {
        return a;
    }
    gcd(b, a % b)
}
