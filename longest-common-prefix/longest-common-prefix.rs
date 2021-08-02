impl Solution {
    pub fn longest_common_prefix(strs: Vec<String>) -> String {
        let mut s = String::from("");
        let mut iters:Vec<_> = strs[1..].iter().map(|x| x.chars()).collect();
        
        'outer: for c in strs[0].chars(){
            'inner: for i in (0..strs.len()-1){
                match iters[i].next(){
                    Some(x) if x != c => break 'outer,
                    None => break 'outer,
                    _ => (),
                }
            }
            s.push(c);
        }
        
        return s;
    }
}