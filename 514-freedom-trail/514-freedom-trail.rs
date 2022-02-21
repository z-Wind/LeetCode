use std::collections::HashMap;

impl Solution {
    pub fn find_rotate_steps(ring: String, key: String) -> i32 {
        let mut ring_vec = vec![Vec::new(); 26];
        for (i, c) in ring.bytes().enumerate() {
            let idx = (c - b'a') as usize;
            ring_vec[idx].push(i);
        }
        let key: Vec<u8> = key.bytes().collect();

        let mut map = HashMap::new();
        find_rotate_steps(&ring_vec, ring.len(), &key, 0, 0, &mut map)
    }
}

fn find_rotate_steps(
    ring: &Vec<Vec<usize>>,
    n: usize,
    key: &Vec<u8>,
    ring_idx: usize,
    key_idx: usize,
    map: &mut HashMap<(usize, usize), i32>,
) -> i32 {
    if key_idx == key.len() {
        return 0;
    }
    if let Some(step) = map.get(&(ring_idx, key_idx)) {
        return *step;
    }

    let c = (key[key_idx] - b'a') as usize;
    let step = ring[c]
        .iter()
        .map(|&next_ring_idx| {
            let mut dis = (next_ring_idx as i32 - ring_idx as i32).abs();
            dis = dis.min(n as i32 - dis);
            1 + dis + find_rotate_steps(ring, n, key, next_ring_idx, key_idx + 1, map)
        })
        .min()
        .unwrap();
    // println!("{},{} => {}", ring_idx, key_idx, step);
    map.insert((ring_idx, key_idx), step);
    step
}
