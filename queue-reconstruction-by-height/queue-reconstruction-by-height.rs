// bubble sort
impl Solution {
    pub fn reconstruct_queue(mut people: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let mut done = false;
        while !done{
            done = true;
            for i in 0..people.len(){
                let mut count = 0;
                for j in 0..i{
                    if people[j][0] >= people[i][0]{
                        count += 1;
                    }
                }
                // println!("{:?} {}:{:?}", people[i], count, people);
                let mut j = i;
                while people[j][1] != count{
                    done = false;
                    if count > people[j][1]{
                        if people[j-1][0] >= people[j][0]{
                            count -= 1;
                        }
                        people.swap(j-1,j);
                        j = j-1;
                    } else {
                        if people[j+1][0] >= people[j][0]{
                            count += 1;
                        }
                        people.swap(j+1,j);
                        j=j+1;
                    }
                    // println!("       {}:{:?}",count,people);
                }
            }
        }
        people
    }
}