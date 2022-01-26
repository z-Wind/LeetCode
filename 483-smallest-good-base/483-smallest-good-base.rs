// Geometric progression
//   1 + r + r^2 + ... + r^(N-1)
// = (r^N - 1)/(r - 1)
// = num

impl Solution {
    pub fn smallest_good_base(n: String) -> String {
        let num: u128 = n.parse().unwrap();
        // println!("{}", num);
        let mut ans = u128::MAX;
        for i in 1.. {
            // println!("{}======", i);
            if sum(2, i) > num {
                break;
            }
            if let Some(x) = binary_search(num, i){
                ans = ans.min(x);
            }
        }
        return ans.to_string();
    }
}

fn binary_search(num: u128, n:u128) -> Option<u128> {
    let mut left = 2;
    let mut right = num - 1;
    while left <= right {
        // println!("{},{}", left, right);
        if sum(left, n) == num {
            return Some(left);
        } else if sum(right, n) == num {
            return Some(right);
        }
        let mid = left + (right - left) / 2;
        let res = sum(mid, n);
        if res == num {
            return Some(mid)
        } else if res > num {
            right = mid - 1;
        } else {
            left = mid + 1;
        }
    }
    None
}

fn sum(r: u128, n: u128) -> u128{
    (r.pow(n as u32) - 1) / (r - 1)
}