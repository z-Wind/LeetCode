use std::fs;
use std::path::PathBuf;

impl Solution {
    pub fn simplify_path(path: String) -> String {
        let mut dirs:Vec<&str> = Vec::new();
        for dir in path.split_terminator("/"){
            match dir{
                "." | "" => continue,
                ".." => {
                    dirs.pop();
                },
                x => dirs.push(x),
            }
        }
        //println!("{:?}", dirs);
        format!("/{}", dirs.join("/"))
    }
}