#[derive(Debug)]
enum State {
    Init,
    Operator,
    Fraction,
}

impl Solution {
    pub fn fraction_addition(expression: String) -> String {
        let expression = expression.as_bytes();
        let n = expression.len();
        let mut result = (0, 1);

        let mut state = State::Init;
        let mut sign = 1;
        let mut i = 0;
        while i < n {
            // println!(
            //     "{} {} sign:{} {:?} => {:?}",
            //     i, expression[i] as char, sign, result, state
            // );
            match state {
                State::Init => match expression[i] {
                    b'0'..=b'9' => state = State::Fraction,
                    b'-' => state = State::Operator,
                    _ => unreachable!(),
                },
                State::Operator => {
                    match expression[i] {
                        b'+' => {
                            sign = 1;
                            state = State::Fraction;
                        }
                        b'-' => {
                            sign = -1;
                            state = State::Fraction;
                        }
                        _ => unreachable!(),
                    }
                    i += 1;
                }
                State::Fraction => {
                    let end = match expression[i..].iter().position(|&c| c == b'+' || c == b'-') {
                        None => n,
                        Some(offset) => i + offset,
                    };
                    let frac = fraction(&expression[i..end]);
                    result = (
                        result.0 * frac.1 + sign * frac.0 * result.1,
                        result.1 * frac.1,
                    );
                    result = reduction(result);

                    i = end;
                    state = State::Operator;
                }
            }
        }

        if result.0 >= 0 {
            format!("{}/{}", result.0, result.1)
        } else {
            format!("-{}/{}", result.0.abs(), result.1)
        }
    }
}

fn fraction(frac: &[u8]) -> (i32, i32) {
    let mut splits = frac.splitn(2, |&c| c == b'/');
    let numerator = splits.next().expect("number");
    let denominator = splits.next().expect("number");

    let numerator = chars_to_int(numerator);
    let denominator = chars_to_int(denominator);

    (numerator, denominator)
}

fn chars_to_int(chars: &[u8]) -> i32 {
    let mut num = 0;
    for c in chars.iter() {
        num = num * 10 + (c - b'0') as i32;
    }

    num
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
