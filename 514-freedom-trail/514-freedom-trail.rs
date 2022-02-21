impl Solution {
    pub fn find_rotate_steps(ring: String, key: String) -> i32 {
        let mut ring_vec = vec![Vec::new(); 26];
        for (i, c) in ring.bytes().enumerate() {
            let idx = (c - b'a') as usize;
            ring_vec[idx].push(i);
        }
        let key: Vec<u8> = key.bytes().collect();

        let mut map = vec![vec![0; key.len()];ring.len()];
        find_rotate_steps(&ring_vec, ring.len(), &key, 0, 0, &mut map)
    }
}

fn find_rotate_steps(
    ring: &Vec<Vec<usize>>,
    n: usize,
    key: &Vec<u8>,
    ring_idx: usize,
    key_idx: usize,
    map: &mut Vec<Vec<i32>>,
) -> i32 {
    if key_idx == key.len() {
        return 0;
    }
    if map[ring_idx][key_idx] != 0 {
        return map[ring_idx][key_idx];
    }

    let c = (key[key_idx] - b'a') as usize;
    map[ring_idx][key_idx] = ring[c]
        .iter()
        .map(|&next_ring_idx| {
            let mut dis = (next_ring_idx as i32 - ring_idx as i32).abs();
            dis = dis.min(n as i32 - dis);
            1 + dis + find_rotate_steps(ring, n, key, next_ring_idx, key_idx + 1, map)
        })
        .min()
        .unwrap();
    map[ring_idx][key_idx]
}
