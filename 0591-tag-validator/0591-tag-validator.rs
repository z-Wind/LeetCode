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

        let mut closed = false;
        let mut i = 0;
        let mut tags: Vec<(usize, usize)> = Vec::new();
        let mut state = State::Char;
        while i < n {
            println!("{} {}:{:?} {:?}", i, code[i] as char, state, tags);
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
                        state = State::TagEnd;
                        i += 1;
                    }
                    b'!' => {
                        if tags.is_empty(){
                            return false;
                        }
                        state = State::CDATAStart;
                        i += 1;
                    }
                    b'A'..=b'Z' => state = State::TagStart,
                    _ => return false,
                },
                State::TagStart => {
                    for j in i + 1..n {
                        match code[j] {
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
                State::TagEnd => match tags.last() {
                    None => return false,
                    Some(&(start, end)) => {
                        let last = i + end - start;
                        if last < n && code[last] == b'>' && code[start..end] == code[i..last] {
                            tags.pop();
                            closed = true;
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
                    let last = i + s.len();
                    if last > n {
                        return false;
                    }

                    if code[i..last] == s[..] {
                        state = State::CDATAContent;
                        i = last;
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
                    let last = i + s.len();
                    if last > n {
                        return false;
                    }

                    if code[i..last] == s[..] {
                        state = State::Char;
                        i = last;
                    } else {
                        state = State::CDATAContent;
                        i += 1;
                    }
                }
            }
        }
        tags.is_empty() && closed
    }
}
