impl Solution {
    pub fn calculate(s: String) -> i32 {
        let mut result = 0;
        let mut op = &b'+';
        let mut curr_num = 0;
        let mut last_num = 0;
        for c in s.as_bytes(){
            match c{
                b'0'..=b'9' => curr_num = curr_num*10 + (c-b'0') as i32,
                b'+' | b'-' | b'*' | b'/' => {
                    match op{
                        b'+' => {
                            result += last_num;
                            last_num = curr_num;
                        },
                        b'-' => {
                            result += last_num;
                            last_num = -curr_num;
                        },
                        b'*' => {
                            last_num *= curr_num;
                        },
                        b'/' => {
                            last_num /= curr_num;
                        },
                        _ => panic!(),
                    }
                    op = c;
                    curr_num = 0;
                },
                b' '=>(),
                _ => panic!(),
            }
        }
        match op{
            b'+' => {
                result += last_num;
                last_num = curr_num;
            },
            b'-' => {
                result += last_num;
                last_num = -curr_num;
            },
            b'*' => {
                last_num *= curr_num;
            },
            b'/' => {
                last_num /= curr_num;
            },
            _ => panic!(),
        }
        result+=last_num;
        result
    }
}
