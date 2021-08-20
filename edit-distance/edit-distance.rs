use std::collections::VecDeque;
use std::mem;

#[derive(Debug)]
enum Action{
    Replace,
    Insert,
    Delete,
}
impl Solution {
    pub fn min_distance(word1: String, word2: String) -> i32 {
        if word1 == word2{
            return 0;
        }
        let mut queue = VecDeque::new();
        let mut word1 = &word1[..];
        let mut word2 = &word2[..];
        
        let mut min_step = i32::MAX;
        let mut step = 0;
        'outer: loop{
            if !word1.is_empty() && !word2.is_empty() && word1[0..1] == word2[0..1]{
                word1 = &word1[1..];
                word2 = &word2[1..];
                continue;
            }
            if !word1.is_empty() && !word2.is_empty() { 
                queue.push_back((&word1[1..], &word2[1..], step, Action::Replace));
            }
            if !word1.is_empty() {
                queue.push_back((&word1[1..], &word2[..], step, Action::Delete));
            }
            if !word2.is_empty(){
                queue.push_back((&word1[..], &word2[1..], step, Action::Insert));
            }
            
            //println!("{:?}", queue);
            if word1.is_empty() && word2.is_empty() && step < min_step{
                min_step = step;
            }
            
            
            loop{
                match queue.pop_front(){
                    None => {
                        //println!("empty queue");
                        break 'outer;
                    },
                    Some((w1, w2, layer, Action::Replace)) => {
                        //println!("Replace");
                        word1 = w1;
                        word2 = w2;
                        step = layer + 1;
                    }
                    Some((w1, mut w2, mut layer, Action::Insert)) => {
                        //println!("Insert");
                        while !w1.is_empty()  && !w2.is_empty() && w1[0..1] != w2[0..1]{
                            w2 = &w2[1..];
                            layer += 1;
                        }
                        word1 = w1;
                        word2 = w2;
                        step = layer + 1;
                    }
                    Some((mut w1, w2, mut layer, Action::Delete)) => {
                        //println!("Delete");
                        while !w1.is_empty() && !w2.is_empty() && w1[0..1] != w2[0..1]{
                            w1 = &w1[1..];
                            layer += 1;
                        }
                        word1 = w1;
                        word2 = w2;
                        step = layer + 1;
                    }
                }
                if step < min_step{
                    break;
                }
            }
            //println!("{:?} ==? {:?}", word1, word2);
        }
        min_step
    }
}