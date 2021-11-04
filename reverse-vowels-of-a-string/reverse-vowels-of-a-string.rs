impl Solution {
    pub fn reverse_vowels(s: String) -> String {
        let mut i = 0;
        let mut j = s.len() - 1;
        let mut s = s.as_bytes().to_vec();
        while i < j {
            let bool_i = matches!(
                s[i],
                b'a' | b'e' | b'i' | b'o' | b'u' | b'A' | b'E' | b'I' | b'O' | b'U'
            );
            let bool_j = matches!(
                s[j],
                b'a' | b'e' | b'i' | b'o' | b'u' | b'A' | b'E' | b'I' | b'O' | b'U'
            );
            if bool_i && bool_j {
                s.swap(i, j);
                i += 1;
                j -= 1;
            }
            if !bool_i {
                i += 1;
            }
            if !bool_j {
                j -= 1;
            }
        }

        String::from_utf8(s).unwrap()
    }
}