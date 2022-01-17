// https://leetcode.com/problems/matchsticks-to-square/discuss/95744/cpp-6ms-solution-with-DFS

impl Solution {
    pub fn makesquare(mut matchsticks: Vec<i32>) -> bool {
        if matchsticks.len() < 4 {
            return false;
        }

        let sum: i32 = matchsticks.iter().sum();
        // sum % 4
        if sum & 0b00 != 0 {
            return false;
        }
        let target = sum / 4;

        matchsticks.sort_unstable_by(|a, b| b.cmp(a));
        if matchsticks[0] > target {
            return false;
        }

        let mut sidesLength = vec![0; 4];
        dfs(&mut sidesLength, &matchsticks, 0, target)
    }
}

fn dfs(sidesLength: &mut Vec<i32>, matches: &Vec<i32>, index: usize, target: i32) -> bool {
    if index == matches.len() {
        return sidesLength[0] == sidesLength[1]
            && sidesLength[1] == sidesLength[2]
            && sidesLength[2] == sidesLength[3];
    }

    for i in 0..4 {
        if sidesLength[i] + matches[index] > target {
            continue;
        }

        //         lets say sides are currently : [5,7,5,9] , the next element is 3.
        //         Therefore we will be applying dfs on the following:
        //         [5+3,7  ,5  ,9  ]
        //         [5  ,7+3,5  ,9  ]
        //         [5  ,7  ,5+3,9  ]
        //         [5  ,7  ,5  ,9+3]
        //         First and third are basically the same, only in a different order. If in the first case, dfs returned false, the third case will return false too.
        //         So we don't run dfs in third case.
        if sidesLength[..i].iter().any(|&x| x == sidesLength[i]) {
            continue;
        }

        sidesLength[i] += matches[index];
        if dfs(sidesLength, matches, index + 1, target) {
            return true;
        }
        sidesLength[i] -= matches[index];
    }
    return false;
}
