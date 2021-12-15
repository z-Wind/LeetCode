impl Solution {
    pub fn original_digits(s: String) -> String {
        let map = vec![
            ("zero", '0', b'z'),
            ("one", '1', b'o'),
            ("two", '2', b'w'),
            ("three", '3', b'h'),
            ("four", '4', b'u'),
            ("five", '5', b'f'),
            ("six", '6', b'x'),
            ("seven", '7', b's'),
            ("eight", '8', b'g'),
            ("nine", '9', b'i'),
        ];

        let mut counts = vec![0; 26];
        for c in s.bytes() {
            let i = (c - b'a') as usize;
            counts[i] += 1;
        }
        
        // check order
        // (z)ero
        // si(x) => (s)even
        // ei(g)ht => t(h)ree
        // t(w)o
        // fo(u)r => (f)ive
        // done all above
        // n(i)ne
        // (o)ne
        let mut ans: Vec<char> = Vec::new();
        for i in vec![0, 6, 7, 8, 3, 2, 4, 5, 9, 1]{
            let (s, num, b) = map[i];
            while counts[(b - b'a') as usize] != 0{
                for c in s.bytes(){
                    let i = (c - b'a') as usize;
                    counts[i] -= 1;
                }
                ans.push(num)
            }    
        }        
        
        ans.sort_unstable();
        ans.into_iter().collect()
    }
}
