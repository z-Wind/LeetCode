use std::collections::HashMap;

impl Solution {
    pub fn is_match(s: String, p: String) -> bool {
        let s = s.as_bytes();
        let p = p.as_bytes();
        
        let mut strIndex = 0;
        let mut patternIndex = 0;
        let mut starIndex = None;
        let mut strRestoreIndex = 0;

        while strIndex < s.len() {
            //println!("strIndex:{}, patternIndex:{}, starIndex:{:?}, strRestoreIndex:{}", strIndex, patternIndex, starIndex, strRestoreIndex);
            if patternIndex < p.len() && (s[strIndex]==p[patternIndex] || p[patternIndex]==b'?') {
                patternIndex+=1;
                strIndex+=1;
            } else if patternIndex < p.len() && p[patternIndex] == b'*' {
                starIndex = Some(patternIndex);
                patternIndex+=1;
                strRestoreIndex = strIndex;
            } else if starIndex.is_some() {
                patternIndex = starIndex.unwrap() + 1;
			    strIndex = strRestoreIndex + 1;
			    strRestoreIndex = strIndex;
            } else {
                return false;
            }
        }

        while patternIndex < p.len() && p[patternIndex] == b'*' {
            patternIndex+=1;
        }
        return patternIndex == p.len();
    }
}

