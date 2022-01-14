/** 
 * The rand7() API is already defined for you.
 * @return a random integer in the range 1 to 7
 * fn rand7() -> i32;
 */

impl Solution {
    pub fn rand10() -> i32 {
        let mut num = None;
        while num.is_none(){
            num = match (rand7(), rand7()){
                (1, 1..=3) => Some(1),
                (1, 4..=6) => Some(2),
                (2, 1..=3) => Some(3),
                (2, 4..=6) => Some(4),
                (3, 1..=3) => Some(5),
                (3, 4..=6) => Some(6),
                (4, 1..=3) => Some(7),
                (4, 4..=6) => Some(8),
                (5, 1..=3) => Some(9),
                (5, 4..=6) => Some(10),
                _ => None,
            };
        }
        num.unwrap()
    }
}