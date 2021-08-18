impl Solution {
    pub fn full_justify(words: Vec<String>, max_width: i32) -> Vec<String> {
        let group = group_by(words, max_width as usize);
        //println!("{:?}", group);

        let n = group.len();
        group
            .into_iter()
            .enumerate()
            .map(|(i, words)| fmt(words, max_width as usize, i == n - 1))
            .collect()
    }
}

fn fmt(words: Vec<String>, max_width: usize, last: bool) -> String {
    let word_n: usize = words.iter().map(|s| s.len()).sum();
    if words.len() == 1 || last {
        let s = words.join(" ");
        return format!("{word:<width$}", word = s, width = max_width);
    }

    let words_len = words.len();
    let rest = max_width - word_n;
    let space_n = rest / (words_len - 1);
    let add_space = rest % (words_len - 1);
    //println!(
    //    "word_n:{}, space_n:{}, add_space:{}",
    //    word_n, space_n, add_space
    //);

    let mut s: String = String::from("");
    for (i, word) in words.into_iter().enumerate() {
        if i < add_space {
            s += &format!(
                "{word:<width$}",
                word = word,
                width = word.len() + space_n + 1
            );
        } else if i < words_len - 1 {
            s += &format!("{word:<width$}", word = word, width = word.len() + space_n);
        } else {
            s += &word;
        }
    }
    s
}

fn group_by(words: Vec<String>, max_width: usize) -> Vec<Vec<String>> {
    let mut group: Vec<Vec<String>> = vec![vec![]];
    let mut len = 0;
    let mut i = 0;
    for word in words {
        len += word.len();
        if len + group[i].len() <= max_width {
            group[i].push(word);
        } else {
            len = word.len();
            group.push(vec![word]);
            i += 1;
        }
    }

    group
}
