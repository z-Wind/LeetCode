impl Solution {
    pub fn makesquare(mut matchsticks: Vec<i32>) -> bool {
        matchsticks.sort_unstable();
        let sum: i32 = matchsticks.iter().sum();
        let target = if sum & 0b00 == 0 {
            sum >> 2
        } else {
            return false;
        };
        
        explore(target, &matchsticks, Vec::new(), 0, 0, 0)
    }
}

fn explore(
    target: i32,
    matchsticks: &Vec<i32>,
    mut temp: Vec<i32>,
    i_start: usize,
    mut cur_sum: i32,
    level: i32,
) -> bool {
    if cur_sum > target {
        return false;
    } else if cur_sum == target {
        temp.extend_from_slice(&matchsticks[i_start..]);
        if level == 2{
            return temp.iter().sum::<i32>() == target;
        } else {
            return explore(target, &temp, Vec::new(), 0, 0, level + 1);
        }
    }
    
    // println!("{},{}:{} {:?} {:?}", target, level, cur_sum, &matchsticks[i_start..], temp);

    for i in i_start..matchsticks.len() {
        if i > i_start && matchsticks[i] == matchsticks[i - 1] {
        } else {
            if explore(
                target,
                matchsticks,
                temp.clone(),
                i + 1,
                cur_sum + matchsticks[i],
                level,
            ) {
                return true;
            }
        }
        temp.push(matchsticks[i]);
    }
    false
}
