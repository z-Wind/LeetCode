use rand::{rngs::ThreadRng, thread_rng, Rng};

struct Solution {
    areas_total: i64,
    areas: Vec<i64>,
    rects: Vec<Vec<i32>>,
    rng: ThreadRng,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl Solution {
    fn new(rects: Vec<Vec<i32>>) -> Self {
        let mut areas_total = 0;
        let mut areas = Vec::with_capacity(rects.len());
        for i in 0..rects.len() {
            let area = get_area(&rects[i]);
            areas_total += area;
            areas.push(area);
        }

        Self {
            areas_total,
            areas,
            rects,
            rng: rand::thread_rng(),
        }
    }

    fn pick(&mut self) -> Vec<i32> {
        // select which rect
        // probability: area/areas_total
        let mut area = self.rng.gen_range(1, self.areas_total + 1);
        let mut i = 0;
        let mut count = 0;
        while i < self.areas.len() {
            count += self.areas[i];
            if count >= area {
                break;
            }
            i += 1;
        }
        // println!("sel:{} {} {} {:?}", i, area, self.areas_total, self.areas);
        
        // gen point
        // probability: 1/area
        let x = self.rng.gen_range(self.rects[i][0], self.rects[i][2] + 1);
        let y = self.rng.gen_range(self.rects[i][1], self.rects[i][3] + 1);

        vec![x, y]
    }
}

fn get_area(rect: &Vec<i32>) -> i64 {
     // include corner point, so add 1
    let w = rect[2] - rect[0] + 1;
    let h = rect[3] - rect[1] + 1;
    w as i64 * h as i64
}


/**
 * Your Solution object will be instantiated and called as such:
 * let obj = Solution::new(rects);
 * let ret_1: Vec<i32> = obj.pick();
 */