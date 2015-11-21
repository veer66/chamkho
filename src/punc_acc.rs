#[derive(Debug, PartialEq)]
enum State {
    Init,
    Space,
    SpaceFinal,
    Text,
    TextFinal
}

#[derive(Debug)]
pub struct PuncAcc {
    i: usize,
    pub start: usize,
    state: State
}

impl PuncAcc {
    pub fn new() -> PuncAcc {
        PuncAcc{start:0, i:0, state:State::Init}
    }

    fn is_space(&self, ch: char) -> bool {
        match ch {
            '"' => true,
            ' ' => true,
            '\n' => true,
            '\t' => true,
            '\r' => true,
            _ => false
        }
    }

    fn to_text_state(&mut self, nch: Option<char>) -> State {
        match nch {
            Some(_nch) => if self.is_space(_nch) {
                State::TextFinal
            } else {
                State::Text
            },
            None => State::TextFinal
        }
    }

    fn to_space_state(&mut self, nch: Option<char>) -> State {
        match nch {
            Some(_nch) => if self.is_space(_nch) {
                State::Space
            } else {
                State::SpaceFinal
            },
            None => State::SpaceFinal
        }
    }
    
    fn to_another_state(&mut self, ch: char, nch: Option<char>) -> State {
        if self.is_space(ch) {
            self.to_space_state(nch)
        } else {
            self.to_text_state(nch)
        }
    }
   
    pub fn transit(&mut self, ch: char, nch: Option<char>) {
        match self.state {
            State::Init => {
                self.start = self.i;
                self.state = self.to_another_state(ch, nch)
            },
            State::Text => { self.state = self.to_another_state(ch, nch); },
            State::TextFinal => {
                self.start = self.i;
                self.state = self.to_space_state(nch);
            }
            State::SpaceFinal => {
                self.start = self.i;
                self.state = self.to_text_state(nch)
            }
            State::Space => if !self.is_space(ch) {
                self.state = self.to_another_state(ch, nch);
            }
        };
        self.i += 1;
    }

    pub fn is_text_final(&self) -> bool {
        self.state == State::TextFinal
    }

    pub fn is_space_final(&self) -> bool {
        self.state == State::SpaceFinal
    }
}