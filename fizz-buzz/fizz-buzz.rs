// https://leetcode.com/problems/fizz-buzz/discuss/89931/Java-4ms-solution-Not-using-%22%22-operation

impl Solution {
    pub fn fizz_buzz(n: i32) -> Vec<String> {
        let mut ans:Vec<String> = Vec::with_capacity(n as usize);
        let mut fizz = 0;
        let mut buzz = 0;
        for i in 1..=n{
            fizz += 1;
            buzz += 1;
            match (fizz, buzz){
                (3,5) => {
                    ans.push(String::from("FizzBuzz"));
                    fizz = 0;
                    buzz = 0;
                },
                (3,_) => {
                    ans.push(String::from("Fizz"));
                    fizz = 0;
                },
                (_,5) => {
                    ans.push(String::from("Buzz"));
                    buzz = 0;
                },
                (_,_) => ans.push(i.to_string()),
            }
        }
        ans
    }
}