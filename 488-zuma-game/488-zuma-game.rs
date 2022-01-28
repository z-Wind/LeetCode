use std::collections::HashMap;

impl Solution {
    pub fn find_min_step(board: String, hand: String) -> i32 {
        let board = board.as_bytes();
        let mut map = HashMap::new();
        for c in [b'R', b'Y', b'B', b'G', b'W']{
            map.insert(c, 0);
        }
        for c in hand.bytes(){
            *map.get_mut(&c).unwrap() += 1;
        }
        let mut ans = i32::MAX;
        // find_min_step(&mut ans, 0, board, &mut map);
        find_min_step_all(&mut ans, 0, board, &mut map);
        if ans == i32::MAX{
            -1
        } else {
            ans
        }
    }
}

fn find_min_step(ans:&mut i32, count:i32, mut board: &[u8], map:&mut HashMap<u8,i32>) {
    if count >= *ans {
        return;
    }
    
    // println!("{}", String::from_utf8(board.to_vec()).unwrap());
    let board = clear(board);
    if board.is_empty(){
        *ans = (*ans).min(count);
        return
    }
    // println!("=> {}:{:?}", String::from_utf8(board.clone()).unwrap(), map);
    
    let n = board.len();
    let mut prev = 0;
    for i in 1..=n  {
        if i < n && board[i] == board[prev] {
            continue;
        }
        
        let x = map.get(&board[prev]).cloned().unwrap();
        if x == 0 {
            prev = i;
            continue;
        }

        map.insert(board[prev], x-1);
        find_min_step(ans, count+1, &[&board[..i], &[board[prev]], &board[i..]].concat(), map);
        map.insert(board[prev], x);
        prev = i;
    }
}

fn find_min_step_all(ans:&mut i32, count:i32, mut board: &[u8], map:&mut HashMap<u8,i32>) {
    if count >= *ans {
        return;
    }
    
    // println!("{}", String::from_utf8(board.to_vec()).unwrap());
    let board = clear(board);
    if board.is_empty(){
        *ans = (*ans).min(count);
        return
    }
    // println!("=> {}:{:?}", String::from_utf8(board.clone()).unwrap(), map);
    
    for i in 0..=board.len() {
        for (&c,&x) in map.clone().iter(){
            if x == 0 {
                continue;
            }
            map.insert(c, x-1);
            find_min_step(ans, count+1, &[&board[..i], &[c], &board[i..]].concat(), map);
            map.insert(c, x);
        }
    }
}

fn clear(board: &[u8]) -> Vec<u8> {
    let mut s = board.to_vec();
    'outer: loop {
        let mut prev = 0;
        for i in 1..=s.len() {
            if i < s.len() && s[i] == s[prev] {
                continue;
            }
            
            // println!("{}", String::from_utf8(s.to_vec()).unwrap());
            // println!("{},{}", prev, i);
            if i - prev >= 3 {
                s = [&s[..prev], &s[i..]].concat();
                continue 'outer;
            };
            prev = i;
        }
        break;
    }
    s
}