// #[derive(Debug, PartialEq, Eq)]
// pub enum NestedInteger {
//   Int(i32),
//   List(Vec<NestedInteger>)
// }
#[derive(Debug)]
struct NestedIterator {
    vec: Vec<NestedInteger>,
    sub: Option<Box<NestedIterator>>,
    temp: Option<i32>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl NestedIterator {
    fn new(mut nestedList: Vec<NestedInteger>) -> Self {
        nestedList.reverse();
        let mut x = Self {
            vec: nestedList,
            sub: None,
            temp: None,
        };
        x.has_next();
        x
    }

    fn next(&mut self) -> i32 {
        // println!("{:?}", self);
        if self.temp.is_some(){
            let x = self.temp.unwrap();
            self.temp = None;
            return x;
        }
        if self.has_next(){
            return self.sub.as_mut().unwrap().next();
        }
        panic!("empty");
    }

    fn has_next(&mut self) -> bool {
        if self.temp.is_some() || 
            (self.sub.is_some() && self.sub.as_mut().unwrap().has_next()){
            return true;
        }
        
        while let Some(nest) = self.vec.pop() {
            match nest{
                NestedInteger::Int(x) => {
                    self.temp = Some(x);
                    return true;
                }
                NestedInteger::List(vec) => {
                    let mut sub = NestedIterator::new(vec);
                    if sub.has_next(){
                        self.sub = Some(Box::new(sub));
                        return true;
                    }
                }
            }
        }
        false
    }
}

/**
 * Your NestedIterator object will be instantiated and called as such:
 * let obj = NestedIterator::new(nestedList);
 * let ret_1: i32 = obj.next();
 * let ret_2: bool = obj.has_next();
 */