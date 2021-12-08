impl Solution {
    pub fn fizz_buzz(n: i32) -> Vec<String> {
        let mut ans:Vec<String> = Vec::with_capacity(n as usize);
        for i in 1..=n{
            match (i%3==0, i%5==0){
                (true,true) => ans.push(String::from("FizzBuzz")),
                (true,false) => ans.push(String::from("Fizz")),
                (false,true) => ans.push(String::from("Buzz")),
                (false,false) => ans.push(i.to_string()),
            }
        }
        ans
    }
}