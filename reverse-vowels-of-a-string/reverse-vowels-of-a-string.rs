impl Solution {
    pub fn reverse_vowels(s: String) -> String {
        let mut i = 0;
        let mut j = s.len() - 1;
        let mut s = s.as_bytes().to_vec();
        
        let is_vowel = |c|{
            matches!(
                c,
                b'a' | b'e' | b'i' | b'o' | b'u' | b'A' | b'E' | b'I' | b'O' | b'U'
            )
        };
        
        while i < j {
            if !is_vowel(s[i]){
                i += 1;
                continue;
            }
            if !is_vowel(s[j]){
                j -= 1;
                continue;
            }
            
            s.swap(i, j);
            i += 1;
            j -= 1;
        }

        String::from_utf8(s).unwrap()
    }
}