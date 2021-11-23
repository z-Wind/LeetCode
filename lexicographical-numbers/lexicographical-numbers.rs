impl Solution {
    pub fn lexical_order(n: i32) -> Vec<i32> {
        let mut ans = Vec::with_capacity(n as usize);
        for i in 1..=9{
            if i > n{
                break;
            }
            ans.push(i);
            lexical_order(&mut ans, i,n);
        }
        ans
    }
}

fn lexical_order(ans: &mut Vec<i32>, head:i32, n: i32) {
    for i in 0..=9{
        let num = head*10+i;
        if num > n{
            break;
        }
        ans.push(num);
        lexical_order(ans, num,n);
    }
}