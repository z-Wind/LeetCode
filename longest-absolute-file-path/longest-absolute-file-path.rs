impl Solution {
    pub fn length_longest_path(input: String) -> i32 {
        let mut path:Vec<_> = vec![];
        let mut len = 0;
        for s in input.split('\n'){
            // println!("{}", s);
            let mut layer = 0;
            let mut start = 0;
            let mut is_file = false;
            for (i,c) in s.chars().enumerate(){
                match c{
                    '\t' => {
                        layer+=1;
                        start = i + 1;
                    },
                    '.' => {
                        is_file = true;
                        break;
                    },
                    _ => (),
                }
            }
            while path.len() > layer{
                path.pop();
            }
            path.push(&s[start..]);
            if is_file{
                // println!("{}", path.join("/"));
                len = len.max(path.join("/").len());
            }
        }
        
        len as i32
    }
}