use std::collections::HashMap;
use std::collections::VecDeque;
impl Solution {
    pub fn find_ladders(begin_word: String, end_word: String, mut word_list: Vec<String>) -> Vec<Vec<String>> {
        let mut ans:Vec<Vec<String>> = Vec::new();
        let mut queue:VecDeque<Vec<String>> = VecDeque::new();
        
        match word_list.iter().position(|x| x == &begin_word){
            None => queue.push_back(vec![begin_word]),
            Some(i) => queue.push_back(vec![word_list.remove(i)]),
        }
        
        'outer: loop{
            let mut remove_i = vec![];
            let len = queue.len();
            if len == 0{
                break;
            }
            // println!("queue:{:?}", queue);
            // println!("word_list:{:?}", word_list);
            for _ in (0..len){
                let mut seq = queue.pop_front().unwrap();
                if !ans.is_empty() && seq.len() >= ans[0].len() {
                    break 'outer;
                }
                // println!("seq:{:?}",seq);
                let key = seq.last().unwrap();
                for i in (0..word_list.len()){
                    if is_single(key.as_bytes(), word_list[i].as_bytes()){
                        if word_list[i] == end_word{
                            seq.push(word_list[i].clone());
                            ans.push(seq);
                            break;
                        }

                        remove_i.push(i);
                        let mut tmp_s = seq.clone();
                        tmp_s.push(word_list[i].clone());
                        queue.push_back(tmp_s);
                    }
                }
            }
            
            remove_i.sort();
            remove_i.dedup();
            for i in remove_i.into_iter().rev(){
                word_list.remove(i);
            }
        }
        ans
    }
}

fn is_single(s1:&[u8], s2:&[u8]) -> bool{
    let mut count = 0;
    for i in (0..s1.len()){
        if s1[i] != s2[i]{
            count += 1;
        }
        if count > 1{
            return false;
        }
    }
    count == 1
}