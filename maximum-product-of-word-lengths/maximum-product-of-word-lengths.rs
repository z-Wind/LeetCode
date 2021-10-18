impl Solution {
    pub fn max_product(words: Vec<String>) -> i32 {
        let n = words.len();
        let mut set:Vec<i32> = vec![0;n];
        
        for i in 0..n{
            for c in words[i].bytes(){
                set[i] |= 1 << (c-b'a');
            }
        }
        
        let mut max_val = 0;
        for i in 0..n{
            let w1 = words[i].len();
            for j in i+1..n{
                if (set[i] & set[j]) != 0{
                    continue;
                }
                let w2 = words[j].len();
                // println!("{},{} => {}", words[i], words[j], w1*w2);
                max_val = max_val.max((w1*w2) as i32);
            }
        }
        
        max_val
    }
}