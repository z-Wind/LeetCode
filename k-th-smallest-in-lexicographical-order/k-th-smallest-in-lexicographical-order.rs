// https://leetcode.com/problems/k-th-smallest-in-lexicographical-order/discuss/92242/ConciseEasy-to-understand-Java-5ms-solution-with-Explaination

impl Solution {
    pub fn find_kth_number(n: i32, k: i32) -> i32 {
        let n = n as i64;
        let mut k = k as i64 - 1;

        let mut curr = 1;
        while k > 0 {
            let steps = calSteps(n, curr, curr + 1);
            if steps <= k {
                curr += 1;
                k -= steps;
            } else {
                curr *= 10;
                k -= 1;
            }
        }
        return curr as i32;
    }
}

fn calSteps(n: i64, mut n1: i64, mut n2: i64) -> i64 {
    let mut steps = 0;
    while n1 <= n {
        // why n+1
        // n = 19
        // n1 = 1, n2 = 2, step = 1
        // n1*10 = 10, n2*10 = 20, step = 19 - 10 + 1 (include self)
        steps += n2.min(n + 1) - n1;
        n1 *= 10;
        n2 *= 10;
    }
    return steps;
}
