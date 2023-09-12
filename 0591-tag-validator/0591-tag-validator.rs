#[derive(Debug)]
enum State {
    Char,
    TagRecognize,
    TagStart,
    TagEnd,
    CDATAStart,
    CDATAContent,
    CDATAEnd,
}

impl Solution {
    pub fn is_valid(code: String) -> bool {
        let code = code.as_bytes();
        let n = code.len();
        if code[0] != b'<' || code[n - 1] != b'>' {
            return false;
        }

        let mut i = 0;
        let mut tags: Vec<(usize, usize)> = Vec::new();
        let mut state = State::Char;
        while i < n {
            // println!("{} {}:{:?} {:?}", i, code[i] as char, state, tags);
            match state {
                State::Char => {
                    match code[i] {
                        b'<' => state = State::TagRecognize,
                        _ => {
                            if tags.is_empty() {
                                return false;
                            }
                        }
                    }
                    i += 1;
                }
                State::TagRecognize => match code[i] {
                    b'/' => {
                        if tags.is_empty() {
                            return false;
                        }

                        state = State::TagEnd;
                        i += 1;
                    }
                    b'!' => {
                        if tags.is_empty() {
                            return false;
                        }

                        state = State::CDATAStart;
                        i += 1;
                    }
                    b'A'..=b'Z' => state = State::TagStart,
                    _ => return false,
                },
                State::TagStart => {
                    for (j, c) in code.iter().enumerate().skip(i + 1) {
                        match c {
                            b'A'..=b'Z' => continue,
                            b'>' => {
                                if j - i > 9 {
                                    return false;
                                }
                                tags.push((i, j));
                                i = j + 1;
                                state = State::Char;
                                break;
                            }
                            _ => {
                                return false;
                            }
                        }
                    }
                }
                State::TagEnd => match tags.pop() {
                    None => return false,
                    Some((start, end)) => {
                        let last = i + end - start;
                        if last < n && code[last] == b'>' && code[start..end] == code[i..last] {
                            i = last + 1;
                            state = State::Char;
                        } else {
                            return false;
                        }
                        if tags.is_empty() && i < n {
                            return false;
                        }
                    }
                },
                State::CDATAStart => {
                    let s = b"[CDATA[";
                    if code[i..].starts_with(&s[..]) {
                        state = State::CDATAContent;
                        i += s.len();
                    } else {
                        return false;
                    }
                }
                State::CDATAContent => match code[i] {
                    b']' => state = State::CDATAEnd,
                    _ => i += 1,
                },
                State::CDATAEnd => {
                    let s = b"]]>";
                    if code[i..].starts_with(&s[..]) {
                        state = State::Char;
                        i += s.len();
                    } else {
                        state = State::CDATAContent;
                        i += 1;
                    }
                }
            }
        }
        tags.is_empty()
    }
}
