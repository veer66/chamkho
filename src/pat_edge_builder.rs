// chamkho - a word breaker written in Rust
// Copyright (C) 2015  Vee Satayamas
//
// This library is free software; you can redistribute it and/or
// modify it under the terms of the GNU Lesser General Public
// License as published by the Free Software Foundation; either
// version 2.1 of the License, or (at your option) any later version.
//
// This library is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU
// Lesser General Public License for more details.
//
// You should have received a copy of the GNU Lesser General Public
// License along with this library; if not, write to the Free Software
// Foundation, Inc., 51 Franklin St, Fifth Floor, Boston, MA  02110-1301  USA
//

use edge::{Edge, EdgeType};
use edge_builder::{EdgeBuildingContext, EdgeBuilder};

#[derive(Debug, PartialEq)]
enum State {
    Init,
    Pat,
    PatFinal,
    NonPat,
    NonPatFinal,
}

type CharPredicate = Fn(char) -> bool;

pub struct PatEdgeBuilder {
    i: usize,
    pub start: usize,
    state: State,
    is_pat_char: Box<CharPredicate>,
    etype: EdgeType,
}

impl PatEdgeBuilder {
    pub fn new(is_pat_char: Box<CharPredicate>, etype: EdgeType) -> PatEdgeBuilder {
        PatEdgeBuilder {
            start: 0,
            i: 0,
            state: State::Init,
            is_pat_char: is_pat_char,
            etype: etype,
        }
    }

    fn to_text_state(&mut self, nch: Option<char>) -> State {
        match nch {
            Some(_nch) => {
                if (self.is_pat_char)(_nch) {
                    State::NonPatFinal
                } else {
                    State::NonPat
                }
            }
            None => State::NonPatFinal,
        }
    }

    fn to_space_state(&mut self, nch: Option<char>) -> State {
        match nch {
            Some(_nch) => {
                if (self.is_pat_char)(_nch) {
                    State::Pat
                } else {
                    State::PatFinal
                }
            }
            None => State::PatFinal,
        }
    }

    fn to_another_state(&mut self, ch: char, nch: Option<char>) -> State {
        if (self.is_pat_char)(ch) {
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
            }
            State::NonPat => {
                self.state = self.to_another_state(ch, nch);
            }
            State::NonPatFinal => {
                self.start = self.i;
                self.state = self.to_space_state(nch);
            }
            State::PatFinal => {
                self.start = self.i;
                self.state = self.to_text_state(nch)
            }
            State::Pat => {
                self.state = self.to_another_state(ch, nch);
            }
        };
        self.i += 1;
    }

    pub fn is_pat_final(&self) -> bool {
        self.state == State::PatFinal
    }
}

impl EdgeBuilder for PatEdgeBuilder {
    fn build(&mut self, context: &EdgeBuildingContext, path: &[Edge]) -> Option<Edge> {
        let next_char = if context.i + 1 == context.text.len() {
            None
        } else {
            Some(context.text[context.i + 1])
        };
        self.transit(context.ch, next_char);
        if self.is_pat_final() {
            let source = path[self.start];
            Some(Edge {
                p: self.start,
                etype: self.etype,
                w: source.w + 1,
                unk: source.unk,
            })
        } else {
            None
        }
    }
}
