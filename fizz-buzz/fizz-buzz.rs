impl Solution {
    pub fn fizz_buzz(n: i32) -> Vec<String> {
        let mut ans:Vec<String> = Vec::with_capacity(n as usize);
        for i in 1..=n{
            ans.push(i.to_string());
        }
        for i in (2..n).step_by(3){
            ans[i as usize] = String::from("Fizz")
        }
        for i in (4..n).step_by(5){
            ans[i as usize] = String::from("Buzz")
        }
        for i in (14..n).step_by(15){
            ans[i as usize] = String::from("FizzBuzz")
        }
        ans
    }
}