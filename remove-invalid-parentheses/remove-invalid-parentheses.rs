// https://leetcode.com/problems/remove-invalid-parentheses/discuss/75027/Easy-Short-Concise-and-Fast-Java-DFS-3-ms-solution

impl Solution {
    pub fn remove_invalid_parentheses(s: String) -> Vec<String> {
        let mut ans:Vec<String> = Vec::new();
        backtrack(s, &mut ans, 0, 0, [b'(', b')']);
        
        ans
    }
}

// i
// j
// ())())
//  ji  
// ())())
// ()()) <= remove ')', i, j equal shift one
// 
//  j  i
// ()())
// befroe i is ok closure
// and we get extra one ')'
// so we can choose any ')' before i include i to remove
//
// if we don't record j and just choose any ')' before i include i to remove                   
//     i           i 
// (a)b)c) -> (ab)c) -> (abc) or (ab)c
//         -> (a)bc) -> (abc) or (a)bc
//                      repeat remove ')' after a
// that's why we need to record j
//
fn backtrack(s:String, ans:&mut Vec<String>, last_i:usize, last_j:usize, par:[u8;2]) {
    // println!("par:{:?}, s:{}, {},{}", par, s, last_i, last_j);
    let mut stack:i32 = 0;
    let b = s.as_bytes();
    for i in (last_i..s.len()) {
        if b[i] == par[0] { 
            stack+=1; 
        } else if b[i] == par[1] { 
            stack-=1;
        }
        if stack >= 0 { 
            continue; 
        }
        
        for j in (last_j..=i){
            if (b[j] == par[1] && (j == last_j || b[j-1] != par[1])){
                let mut temp = s.clone();
                temp.remove(j);
                backtrack(temp, ans, i, j, par);
            }
        }
        // just process extra one par[1]
        return;
    }
    
    let reversed = s.chars().rev().collect::<String>();
    if par[0] == b'(' {// finished left to right
        backtrack(reversed, ans, 0, 0, [b')', b'(']);
    } else {// finished right to left
        ans.push(reversed);
    }
}