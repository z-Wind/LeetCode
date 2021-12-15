// check order
// (z)ero
// t(w)o
// si(x) => (s)even
// ei(g)ht => t(h)ree
// fo(u)r => (f)ive
// done all above
// (o)ne
// n(i)ne
impl Solution {
    pub fn original_digits(s: String) -> String {
        let map = vec![
            ("zero", '0', b'z'),
            ("two", '2', b'w'),
            ("six", '6', b'x'),
            ("seven", '7', b's'),
            ("eight", '8', b'g'),
            ("three", '3', b'h'),
            ("four", '4', b'u'),
            ("five", '5', b'f'),
            ("one", '1', b'o'),    
            ("nine", '9', b'i'),
        ];

        let mut counts = vec![0; 26];
        for c in s.bytes() {
            let i = (c - b'a') as usize;
            counts[i] += 1;
        }
        
        let mut ans: Vec<char> = Vec::new();
        for (s, num, b) in map{
            let k = counts[(b - b'a') as usize];
            if k == 0{
                continue;
            }
            for c in s.bytes(){
                let i = (c - b'a') as usize;
                counts[i] -= k;
            }
            for _ in 0..k{
                ans.push(num);
            }
        }        
        
        ans.sort_unstable();
        ans.into_iter().collect()
    }
}
