// https://leetcode.com/problems/remove-duplicate-letters/discuss/76769/Java-solution-using-Stack-with-comments

impl Solution {
    pub fn remove_duplicate_letters(s: String) -> String {
        let mut res = [0; 26]; // will contain number of occurences of character (i+'a')
        let mut visited = [false; 26]; // will contain if character ('a' + i) is present in current result Stack
        for c in s.chars() {
            // count number of occurences of character
            res[(c as u8 - b'a') as usize] += 1;
        }
        let mut sb: Vec<char> = Vec::new(); // answer stack
        for c in s.chars() {
            let index = (c as u8 - b'a') as usize;
            res[index] -= 1; // decrement number of characters remaining in the string to be analysed
            if (visited[index]) {
                // if character is already present in stack, dont bother
                continue;
            }
            // if current character is smaller than last character in stack which occurs later in the string again
            // it can be removed and  added later e.g stack = bc remaining string abc then a can pop b and then c
            while sb.len() > 0
                && c < sb[sb.len() - 1]
                && res[(sb[sb.len() - 1] as u8 - b'a') as usize] != 0
            {
                let last_c = (sb[sb.len() - 1] as u8 - b'a') as usize;
                visited[last_c] = false;
                sb.pop();
            }
            sb.push(c); // add current character and mark it as visited
            visited[index] = true;
        }

        sb.into_iter().collect()
    }
}
