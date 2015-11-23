/*
 * chamkho - a word breaker written in Rust
 * Copyright (C) 2015  Vee Satayamas
 *
 * This library is free software; you can redistribute it and/or
 * modify it under the terms of the GNU Lesser General Public
 * License as published by the Free Software Foundation; either
 * version 2.1 of the License, or (at your option) any later version.
 *
 * This library is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU
 * Lesser General Public License for more details.
 *
 * You should have received a copy of the GNU Lesser General Public
 * License along with this library; if not, write to the Free Software
 * Foundation, Inc., 51 Franklin St, Fifth Floor, Boston, MA  02110-1301  USA
 */

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
