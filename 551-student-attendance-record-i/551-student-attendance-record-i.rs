enum State {
    Init,
    Absent,
    Late,
    Present,
    End,
}

impl Solution {
    pub fn check_record(s: String) -> bool {
        let mut absent_count = 0;
        let mut late_count = 0;
        let mut pre_state = State::Init;

        for c in s.chars() {
            match c {
                'A' => {
                    absent_count += 1;
                    if absent_count >= 2 {
                        return false;
                    }

                    pre_state = State::Absent;
                }
                'L' => {
                    if matches!(pre_state, State::Late) {
                        late_count += 1;
                        if late_count >= 3 {
                            return false;
                        }
                    } else {
                        late_count = 1;
                    }

                    pre_state = State::Late;
                }
                'P' => {
                    pre_state = State::Present;
                }
                _ => panic!("impossible"),
            }
        }

        true
    }
}
