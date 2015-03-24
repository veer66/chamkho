#[derive(Debug, PartialEq)]
enum State {
    Init,
    Space,
    Text,
    TextFinal
}

#[derive(Debug)]
pub struct SpaceAcc {
    i: usize,
    start: usize,
    state: State
}

impl SpaceAcc {
    pub fn new() -> SpaceAcc {
        SpaceAcc{start:0, i:0, state:State::Init}
    }

    fn is_space(&self, ch: char) -> bool {
        match ch {
            ' ' => true,
            '\n' => true,
            '\t' => true,
            '\r' => true,
            _ => false
        }
    }
    
    pub fn transit(&mut self, ch: char, nch: Option<char>) {
        match self.state {
            State::Init => {
                self.start = self.i;
                self.state = if self.is_space(ch) {
                    State::Space
                } else {
                    match nch {
                        Some(_nch) => if self.is_space(_nch) {
                            State::TextFinal
                        } else {
                            State::Text
                        },
                        None => State::TextFinal,
                    }
                }
            },
            State::Text => if self.is_space(ch) {
                self.state = State::Space
            } else {
                match nch {
                    Some(_nch) => if self.is_space(_nch) {
                        self.state = State::TextFinal
                    },
                    None => self.state = State::TextFinal
                }
            },
            State::TextFinal => self.state = State::Space, 
            State::Space => if !self.is_space(ch) {
                self.start = self.i;
                self.state = match nch {
                    Some(_nch) => if self.is_space(_nch) {
                        State::TextFinal
                    } else {
                        State::Text
                    },
                    None => State::TextFinal
                }
            }
        };
        self.i += 1
    }

    pub fn is_final(&self) -> bool {
        self.state == State::TextFinal
    }
}