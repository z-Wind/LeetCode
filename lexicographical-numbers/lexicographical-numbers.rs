impl Solution {
    pub fn lexical_order(n: i32) -> Vec<i32> {
        let mut ans = vec![];
        for i in 1..=9{
            if i > n{
                break;
            }
            ans.push(i);
            ans.append(&mut lexical_order(i,n));
        }
        ans
    }
}

fn lexical_order(head:i32, n: i32) -> Vec<i32> {
    let mut ans = vec![];
    for i in 0..=9{
        let num = head*10+i;
        if num > n{
            break;
        }
        ans.push(num);
        ans.append(&mut lexical_order(num,n));
    }
    ans
}