// https://leetcode.com/problems/mini-parser/discuss/86066/An-Java-Iterative-Solution

// #[derive(Debug, PartialEq, Eq)]
// pub enum NestedInteger {
//   Int(i32),
//   List(Vec<NestedInteger>)
// }
impl Solution {
    pub fn deserialize(s: String) -> NestedInteger {
        if &s[0..1] != "[" {
            return NestedInteger::Int(s.parse().expect("should be a number"));
        }

        let mut stack: Vec<NestedInteger> = Vec::new();
        let mut curr: Option<Vec<NestedInteger>> = None;
        let mut l = 0; // l shall point to the start of a number substring;
                       // r shall point to the end+1 of a number substring
        for (r, ch) in s.chars().enumerate() {
            match ch {
                '[' => {
                    if curr.is_some() {
                        stack.push(NestedInteger::List(curr.unwrap()));
                    }
                    curr = Some(Vec::new());
                    l = r + 1;
                }
                ']' => {
                    let num = &s[l..r];
                    if !num.is_empty() {
                        curr = curr.map(|mut e| {
                            e.push(NestedInteger::Int(num.parse().expect("should be a number")));
                            e
                        });
                    }
                    if let Some(NestedInteger::List(mut pop)) = stack.pop() {
                        pop.push(NestedInteger::List(curr.unwrap()));
                        curr = Some(pop);
                    }
                    l = r + 1;
                }
                ',' => {
                    if &s[r - 1..r] != "]" {
                        let num = &s[l..r];
                        curr = curr.map(|mut e| {
                            e.push(NestedInteger::Int(num.parse().expect("should be a number")));
                            e
                        });
                    }
                    l = r + 1;
                }
                _ => (),
            }
        }

        return NestedInteger::List(curr.unwrap());
    }
}
