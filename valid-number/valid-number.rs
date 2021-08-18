enum State {
    Start,
    Normal,
    AfterSign,
    
    DotFirst,
    AfterDotFirst,
    AfterDot,
    
    AfterEFirst,
    AfterESign,
    AfterE,
    
    AfterDotEFirst,
    AfterDotE,
}

impl Solution {
    pub fn is_number(s: String) -> bool {
        let mut state = State::Start;
        let mut is_valid = false;
        for c in s.chars(){
            match state{
                State::Start => match c{
                    '0'..='9' => {
                        is_valid = true;
                        state = State::Normal;
                    },
                    '+' | '-' => state = State::AfterSign,
                    '.' => state = State::DotFirst,
                    _ => return false,
                },
                State::DotFirst => match c{
                    '0'..='9' => {
                        is_valid = true;
                        state = State::AfterDot;
                    },
                    _ => return false,
                },
                State::AfterSign => match c{
                    '0'..='9' => {
                        is_valid = true;
                        state = State::Normal;
                    },
                    '.' => state = State::AfterDotFirst,
                    _ => return false,
                }
                State::Normal => match c{
                    '0'..='9' => is_valid = true,
                    '.' => state = State::AfterDotFirst,
                    'e' | 'E' => {
                        is_valid = false;
                        state = State::AfterEFirst;
                    },
                    _ => return false,
                },
                State::AfterDotFirst => match c{
                    '0'..='9' => {
                        is_valid = true;
                        state = State::AfterDot;
                    },
                    'e' | 'E' => {
                        is_valid = false;
                        state = State::AfterEFirst;
                    },
                    _ => return false,
                },
                State::AfterDot => match c{
                    '0'..='9' => is_valid = true,
                    'e' | 'E' => {
                        is_valid = false;
                        state = State::AfterDotEFirst;
                    }
                    _ => return false,
                },
                State::AfterEFirst => match c{
                    '0'..='9' => {
                        is_valid = true;
                        state = State::AfterE;
                    }
                    '+' | '-' => state = State::AfterESign,
                    _ => return false,
                },
                State::AfterESign => match c{
                    '0'..='9' => {
                        is_valid = true;
                        state = State::AfterE;
                    },
                    _ => return false,
                },
                State::AfterE => match c{
                    '0'..='9' => is_valid = true,
                    _ => return false,
                },
                State::AfterDotEFirst => match c{
                    '0'..='9' => {
                        is_valid = true;
                        state = State::AfterDotE;
                    }
                    '+' | '-' => state = State::AfterDotE,
                    _ => return false,
                },
                State::AfterDotE => match c{
                    '0'..='9' => is_valid = true,
                    _ => return false,
                },
            }
        }
        is_valid
    }
}