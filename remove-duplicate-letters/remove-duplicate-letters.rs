use std::collections::VecDeque;
impl Solution {
    pub fn remove_duplicate_letters(s: String) -> String {
        let mut len = 0;
        let mut count_c = vec![VecDeque::new();26];
        let mut select = [false;26];
        for (i,c) in s.bytes().enumerate(){
            let ci = (c - b'a') as usize;
            if count_c[ci].is_empty(){
                len+=1;
            }
            count_c[ci].push_back(i);
        }
        
        let mut temp = vec![b'a';len];
        match remove_duplicate_letters(&mut temp, 0, count_c, &mut select){
            Some(ans) => String::from_utf8(ans).unwrap(),
            _ => String::new(),
        }
    }
}

fn remove_duplicate_letters(temp:&mut Vec<u8>, start:usize, count_c: Vec<VecDeque<usize>>, select:&mut [bool;26]) -> Option<Vec<u8>>{
    // println!("{}: {:?}", String::from_utf8(temp.clone()).unwrap(), count_c);
    if start == temp.len(){
        return Some(temp.clone());
    }
    
    'outer: for i in 0..count_c.len(){
        if select[i] || count_c[i].is_empty(){
            continue;
        }
        let mut counts = count_c.clone();
        for j in 0..counts.len(){
            if i == j || select[j] || counts[j].is_empty(){
                continue;
            }
            // println!("{}:{:?}, {}:{:?}", i, count_c[i], j, counts[j]);
            while counts[j][0] < count_c[i][0]{
                counts[j].pop_front();
                if counts[j].is_empty(){
                    continue 'outer;   
                }
            }
        }
        select[i] = true;
        temp[start] += (i as u8);
        match remove_duplicate_letters(temp,start+1,counts,select){
            None => (),
            x => return x,
        }
        select[i] = false;
        temp[start] -= (i as u8);
    }
    None
}
