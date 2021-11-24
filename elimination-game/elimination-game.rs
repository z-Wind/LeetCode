// 1 2
//   1   => n/2
//
// 1 2 3
//   1   => n/2
//
// 1 2 3 4
//   1   2  => n/2
//   1      => (n+1)/2
//
// 1 2 3 4 5
//   1   2    => n/2
//   1        => (n+1)/2
//
// 1 2 3 4 5 6
//   1   2   3  => n/2
//       1      => n/2
//
// 1 2 3 4 5 6 7 8 9 10 11 12
//   1   2   3   4    5     6   => n/2
//   1       2        3         => (n+1)/2
//           1                  => n/2
//
// 1  => 1
// 2  => 2
// 3  => 2
// 4  => 2
// 5  => 2
// 6  => 4
// 7  => 4
// 8  => 6
// 9  => 6
// 10 => 8
// 12 => 6

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
    pub fn last_remaining(mut n: i32) -> i32 {
        let mut order = Order::LtoR;
        let mut cal = vec![];
        
        // top to down
        while n >= 2{
            // println!("{}:{:?}", n, order);
            match (&order, n%2){
                (Order::LtoR, 0) => {
                    cal.push(0);
                    n>>=1;
                },
                (Order::LtoR, 1) => {
                    cal.push(0);
                    n>>=1;
                },
                (Order::RtoL, 1) => {
                    cal.push(0);
                    n>>=1;
                },
                (Order::RtoL, 0) => {
                    cal.push(-1);
                    n = (n+1)>>1;
                }
                _ => panic!("impossible")
            }
            order = reverse_order(order);
        }
        // println!("{}:{:?}",n,cal);
        
        // down to top
        while let Some(x) = cal.pop(){
            n = (n<<1)+x;
        }
        n
    }
}