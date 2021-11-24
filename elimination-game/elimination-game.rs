// https://leetcode.com/problems/elimination-game/discuss/87119/JAVA%3A-Easiest-solution-O(logN)-with-explanation
// head = 1, left = true, step = 1 (times 2 each turn), remaining = n(24)
// 1 2 3 4 5 6 7 8 9 10 11 12 13 14 15 16 17 18 19 20 21 22 23 24
//
// head = 2, left = false, step = 1 * 2 = 2, remaining = remaining / 2 = 12
// 1 2 3 4 5 6 7 8 9 10 11 12 13 14 15 16 17 18 19 20 21 22 23 24 - > 2 4 6 8 10 12 14 16 18 20 22 24
//
// head = 2, left = true, step = 2 * 2 = 4, remaining = remaining / 2 = 6
// 2 4 6 8 10 12 14 16 18 20 22 24 - > 2 6 10 14 18 22
//
// head = 6, left = false, step = 4 * 2 = 8, remaining = remaining / 2 = 3
// 2 6 10 14 18 22 - > 6 14 22
//
// head = 14, left = true, step = 8 * 2 = 16, remaining = remaining / 2 = 1
// 6 14 22 - > 14

#[derive(Debug)]
enum Order{
    LtoR,
    RtoL,
}
fn reverse_order(order:Order) -> Order{
    match order{
        Order::LtoR => Order::RtoL,
        Order::RtoL => Order::LtoR,
    }
}
impl Solution {
    pub fn last_remaining(n: i32) -> i32 {
        let mut head = 1;
        let mut step = 1;
        let mut order = Order::LtoR;
        let mut remaining = n;
        while remaining > 1{
            // println!("head = {}, order = {:?}, step = {}, remaining = {}", head, order, step, remaining);
            match (&order, remaining%2){
                (Order::LtoR, 0) => {
                    head += step;
                    step <<= 1;
                },
                (Order::LtoR, 1) => {
                    head += step;
                    step <<= 1;
                },
                (Order::RtoL, 1) => {
                    head += step;
                    step <<= 1;
                },
                (Order::RtoL, 0) => {
                    step <<= 1;
                }
                _ => panic!("impossible")
            }
            order = reverse_order(order);
            remaining /= 2;
        }
        head
    }
}