impl Solution {
    pub fn add_strings(num1: String, num2: String) -> String {
        let mut sum = String::new();
        let mut num1 = num1.bytes().rev();
        let mut num2 = num2.bytes().rev();
        let mut carry = 0;
        loop{
            let mut c = match (num1.next(),num2.next()){
                (Some(c1),Some(c2)) => c1 - b'0' + c2 - b'0' + carry,
                (Some(c),_) | (_,Some(c)) => c - b'0' + carry,
                (None, None) => break,
            };
            carry = c / 10;
            c %= 10;
            sum.push((c + b'0') as char);
        }
        if carry == 1{
            sum.push('1');
        }
        sum.chars().rev().collect()
    }
}