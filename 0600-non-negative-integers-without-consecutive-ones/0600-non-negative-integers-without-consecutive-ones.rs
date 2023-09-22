//        1
//       / \
//      10  11(NG)
//     /   \
//   100    101
//   /  \    /   \
// 1000 1001 1010 1011(NG)

impl Solution {
    pub fn find_integers(n: i32) -> i32 {
        // add case 0
        1 + solve(1, n)
    }
}

fn solve(base: i32, n: i32) -> i32 {
    if base > n {
        return 0;
    }
    // println!("{} {}", base, n);

    if base & 1 == 1 {
        1 + solve(base << 1, n)
    } else {
        1 + solve(base << 1, n) + solve((base << 1) + 1, n)
    }
}
