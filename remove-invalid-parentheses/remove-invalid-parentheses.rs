use std::collections::HashSet;

impl Solution {
    pub fn remove_invalid_parentheses(s: String) -> Vec<String> {
        let mut max_left = 0;
        let mut max_right = 0;
        let mut count:i32 = 0;
        for c in s.chars(){
            match c{
                '(' => {
                    count+=1;
                },
                ')' => {
                    if count <= 0{
                        max_right += 1;
                        count = 0;
                    } else {
                        count-=1;   
                    }
                },
                _ => (),
            }
        }
        if count >= 0{
            max_left += count;
        } else {
            max_right -= count;
        }
        
        // println!("{}", max_remove);
        if max_left + max_right == 0{
            return vec![s];
        } else if max_left + max_right == s.len() as i32{
            return vec![String::new()];
        }
        
        let mut ans:HashSet<String> = HashSet::new();
        let mut temp = String::new();
        backtrack(&mut ans, &mut temp, &s, 0, 0, max_left, max_right);
        ans.into_iter().collect()
    }
}

fn backtrack(ans:&mut HashSet<String>, temp:&mut String, s:&str, start:usize, count:i32, max_left:i32, max_right:i32){
    // println!("{}:{} {}", count, temp, max_remove);
    if start == s.len(){
        if count == 0{
            ans.insert(temp.clone());
        }
        return;
    }
    if max_left < 0 || max_right < 0{
        return;
    }
    match &s[start..start+1]{
        "(" => {
            temp.push('(');
            backtrack(ans, temp, s, start+1, count+1, max_left, max_right);
            temp.pop();
            backtrack(ans, temp, s, start+1, count, max_left-1, max_right);
        },
        ")" => {
            if count > 0{
                temp.push(')');
                backtrack(ans, temp, s, start+1, count-1, max_left, max_right);
                temp.pop();
            }
            backtrack(ans, temp, s, start+1, count, max_left, max_right-1);
        },
        c => {
            temp.push_str(c);
            backtrack(ans, temp, s, start+1, count, max_left, max_right);
            temp.pop();
        },
    }
}