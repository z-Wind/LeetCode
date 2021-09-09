// Facts:
// 1. A cannot reach B
// 2. There are C1,C2, ..., Ck between A and B
// 3. A can reach C1, C2, ..., Ck
// A --- C1 --- C2  --- ... Ck --- B
// Proof by contradiction:
// Assume: C1 can reach B
// A can reach C1 (by Fact3) & C1 can reach B => A can reach B (Contradict with Fact1 !)
// => assumption is wrong, C1 cannot reach B
// Same proof could be applied to C2 ~ Ck
// => any station between A and B that A can reach cannot reach B
impl Solution {
    pub fn can_complete_circuit(gas: Vec<i32>, cost: Vec<i32>) -> i32 {
        let mut sumGas = 0;
        let mut sumCost = 0;
        let mut start = 0;
        let mut tank = 0;
        for i in (0..gas.len()) {
            sumGas += gas[i];
            sumCost += cost[i];
            tank += gas[i] - cost[i];
            if (tank < 0) {
                start = i + 1;
                tank = 0;
            }
        }
        if (sumGas < sumCost) {
            return -1;
        } else {
            return start as i32;
        }
    }
}