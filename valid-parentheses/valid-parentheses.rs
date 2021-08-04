use std::collections::LinkedList;
use std::collections::HashMap;

impl Solution {
    pub fn is_valid(s: String) -> bool {
        let mut d:LinkedList<char> = LinkedList::new();
        let mut mapping = HashMap::new();
        mapping.insert('(',')');
        mapping.insert('{','}');
        mapping.insert('[',']');
        
        for c in s.chars(){
            match c{
                '(' | '{' | '[' => d.push_front(*mapping.get(&c).unwrap()),
                ')' | '}' | ']' => {
                        let p = d.pop_front();
                        if p.is_none() || p.unwrap() != c{
                            return false;
                        }                        
                    },
                _ => (),
            }
        }
        
        return d.len() == 0;
    }
}