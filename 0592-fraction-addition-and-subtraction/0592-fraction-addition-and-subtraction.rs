// https://leetcode.com/problems/fraction-addition-and-subtraction/solutions/103384/small-simple-c-java-python/

impl Solution {
    pub fn fraction_addition(expression: String) -> String {
        let expression = expression.replace('-', "+-");
        let expression = expression.trim_start_matches('+');
        let mut sc = expression.split(['+', '/']);
        // println!("{:?}", sc);
        let mut result = (0, 1);

        while let (Some(a), Some(b)) = (sc.next(), sc.next()) {
            // println!("{},{}", a, b);
            let a: i32 = a.parse().expect("number");
            let b: i32 = b.parse().expect("number");
            result.0 = result.0 * b + result.1 * a;
            result.1 *= b;

            result = reduction(result);
        }

        format!("{}/{}", result.0, result.1)
    }
}

fn reduction(x: (i32, i32)) -> (i32, i32) {
    let gcd = gcd(x.1, x.0.abs());
    (x.0 / gcd, x.1 / gcd)
}

fn gcd(a: i32, b: i32) -> i32 {
    if b == 0 {
        return a;
    }

    gcd(b, a % b)
}
