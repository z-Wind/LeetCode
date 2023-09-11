// https://leetcode.com/problems/erect-the-fence/solutions/2828904/python3-monotone-chain-with-detailed-explanations-o-nlogn/

use std::collections::HashSet;

impl Solution {
    pub fn outer_trees(trees: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        if trees.len() <= 3 {
            return trees;
        }

        let mut points = trees;
        // sort points, so we are moving from left to right from bottom to top
        points.sort_unstable();
        // println!("{:?}", points);
        let leftToRight = construct_half_hull(points.iter());
        // reverse points, so we are moving from right to left
        let rightToLeft = construct_half_hull(points.iter().rev());

        // it is posible that the top and bottom parts have same points (e.g., all points form a line)
        // we remove the duplicated points using a set
        let mut result = leftToRight;
        result.extend(rightToLeft);
        let result: HashSet<_> = result.into_iter().collect();
        result.into_iter().collect()
    }
}

// https://en.wikipedia.org/wiki/Cross_product
fn cross_product(p1: &[i32], p2: &[i32], p3: &[i32]) -> i32 {
    // V1 = (P1,P2)
    // V2 = (P2,P3)
    // V1 = (a,b), V2 = (c,d)
    // V1 X V2 = a*d - b*c

    let a = p2[0] - p1[0];
    let b = p2[1] - p1[1];
    let c = p3[0] - p2[0];
    let d = p3[1] - p2[1];

    let result = a * d - b * c;
    // println!("{:?},{:?},{:?} => {}", p1, p2, p3, result);
    result
}

fn construct_half_hull<'a>(points: impl Iterator<Item = &'a Vec<i32>>) -> Vec<Vec<i32>> {
    let mut stack: Vec<Vec<i32>> = Vec::new();
    for p in points {
        // println!("p:{:?}", p);
        // if the chain formed by the current point
        // and the last two points in the stack is not counterclockwise, pop it
        while stack.len() >= 2
            && cross_product(&stack[stack.len() - 2], &stack[stack.len() - 1], p) < 0
        {
            stack.pop();
        }
        // append the current point.
        stack.push(p.clone());
        // println!("stack:{:?}", stack);
    }
    stack
}
