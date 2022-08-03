use std::collections::HashMap;
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

#[derive(Default)]
struct Codec {
	map: HashMap<u64, String>,
}

/** 
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl Codec {
    fn new() -> Self {
        Default::default()
    }
	
    // Encodes a URL to a shortened URL.
    fn encode(&mut self, longURL: String) -> String {
        let hex_digest = calculate_hash(&longURL);
        self.map.insert(hex_digest, longURL);
        let url = format!("http://tinyurl.com/{:x}", hex_digest);
        println!("{}", url);
        url
    }
	
    // Decodes a shortened URL to its original URL.
    fn decode(&self, shortURL: String) -> String {
        let hex_digest = shortURL.replace("http://tinyurl.com/", "");
        let hex_digest = match u64::from_str_radix(&hex_digest, 16) {
            Ok(num) => num,
            Err(e) => return String::new(),
        };
        match self.map.get(&hex_digest) {
            None => String::new(),
            Some(url) => url.to_string(),
        }
    }
}

fn calculate_hash<T: Hash>(t: &T) -> u64 {
    let mut s = DefaultHasher::new();
    t.hash(&mut s);
    s.finish()
}

/**
 * Your Codec object will be instantiated and called as such:
 * let obj = Codec::new();
 * let s: String = obj.encode(strs);
 * let ans: VecVec<String> = obj.decode(s);
 */