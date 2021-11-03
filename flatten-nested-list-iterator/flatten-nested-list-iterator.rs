// https://leetcode.com/problems/flatten-nested-list-iterator/discuss/80147/Simple-Java-solution-using-a-stack-with-explanation

// #[derive(Debug, PartialEq, Eq)]
// pub enum NestedInteger {
//   Int(i32),
//   List(Vec<NestedInteger>)
// }
#[derive(Debug)]
struct NestedIterator {
    stack: Vec<NestedInteger>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl NestedIterator {
    fn new(mut nestedList: Vec<NestedInteger>) -> Self {
        nestedList.reverse();
        Self {
            stack: nestedList,
        }
    }

    fn next(&mut self) -> i32 {
        if let Some(NestedInteger::Int(x)) = self.stack.pop(){
            return x;
        }
        panic!("impossible");
    }

    fn has_next(&mut self) -> bool {
        match self.stack.pop(){
            None => false,
            Some(NestedInteger::List(vec)) => {
                for ele in vec.into_iter().rev(){
                    self.stack.push(ele);
                }
                self.has_next()
            }
            Some(x) => {
                self.stack.push(x);
                true
            },
        }
    }
}

/**
 * Your NestedIterator object will be instantiated and called as such:
 * let obj = NestedIterator::new(nestedList);
 * let ret_1: i32 = obj.next();
 * let ret_2: bool = obj.has_next();
 */