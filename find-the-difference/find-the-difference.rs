impl Solution {
    pub fn find_the_difference(s: String, t: String) -> char {
        let mut counts = vec![0;26];
        for c in s.bytes(){
            let i = (c-b'a') as usize;
            counts[i]+=1;
        }
        for c in t.bytes(){
            let i = (c-b'a') as usize;
            if counts[i] == 0{
                return c as char;
            }
            counts[i]-=1;
        }
        panic!("impossible");
    }
}