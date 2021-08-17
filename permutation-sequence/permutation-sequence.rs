use std::char;
impl Solution {
    pub fn get_permutation(n: i32, k: i32) -> String {
        let mut table:Vec<char> = ('1'..=char::from_digit(n as u32, 10).unwrap()).collect();
        let mut ans:Vec<char> = vec![];
        for i in (0..n).rev(){
            let index = ((k-1) / factorial(i)) as usize % table.len();
            //println!("index;{}, table:{:?}", index, table);
            ans.push(table[index]);
            table.remove(index);
        }
        
        ans.into_iter().collect()
    }
}

fn factorial(n: i32) -> i32 {
    match n{
        0 => 1,
        1 => 1,
        2 => 2,
        3 => 6,
        4 => 24,
        5 => 120,
        6 => 720,
        7 => 5040,
        8 => 40320,
        9 => 362880,
        _ => (1..=n).product(),    
    }
}