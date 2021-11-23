// #[derive(Debug, PartialEq, Eq)]
// pub enum NestedInteger {
//   Int(i32),
//   List(Vec<NestedInteger>)
// }
impl Solution {
    pub fn deserialize(s: String) -> NestedInteger {
        deserialize(&s)
    }
}

fn deserialize(s: &str) -> NestedInteger {
    // println!("'{}'", s);
    match &s[0..1]{
        "[" => {
            let mut parn = 1;
            let mut start = 1;
            let mut v:Vec<NestedInteger> = Vec::new();
            for i in 1..s.len(){
                match &s[i..i+1]{
                    "[" => parn+=1,
                    "]" => {
                        parn-=1;
                        if parn == 0 && start != i{
                            v.push(deserialize(&s[start..i]));
                        }
                    },
                    "," => {
                        if parn == 1{
                            v.push(deserialize(&s[start..i]));
                            start = i+1;
                        }
                    },
                    _ => (),
                }
            }
            return NestedInteger::List(v);
        },
        _ => {
            return NestedInteger::Int(s.parse().expect("should be a number"));
        },
    } 
}