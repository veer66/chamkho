use std::collections::HashSet;

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
    state: State,
    split_char: HashSet<char>
}

impl PuncAcc {
    pub fn new() -> PuncAcc {
        let mut split_char = HashSet::new();
        split_char.insert('"');
        split_char.insert('[');
        split_char.insert(']');
        split_char.insert('(');
        split_char.insert(')');
        split_char.insert(':');
        split_char.insert(' ');
        split_char.insert('\n');
        split_char.insert('\t');
        split_char.insert('\r');
        PuncAcc{start:0, i:0, state:State::Init, split_char: split_char}
    }

    fn is_split_char(&self, ch: char) -> bool {
        self.split_char.contains(&ch)
    }

    fn to_text_state(&mut self, nch: Option<char>) -> State {
        match nch {
            Some(_nch) => if self.is_split_char(_nch) {
                State::TextFinal
            } else {
                State::Text
            },
            None => State::TextFinal
        }
    }

    fn to_space_state(&mut self, nch: Option<char>) -> State {
        match nch {
            Some(_nch) => if self.is_split_char(_nch) {
                State::Space
            } else {
                State::SpaceFinal
            },
            None => State::SpaceFinal
        }
    }
    
    fn to_another_state(&mut self, ch: char, nch: Option<char>) -> State {
        if self.is_split_char(ch) {
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
            State::Space => if !self.is_split_char(ch) {
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