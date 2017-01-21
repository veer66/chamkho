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

use edge_builder::{EdgeBuilder, EdgeBuildingContext};
use dict_edge_builder::DictEdgeBuilder;
use unk_edge_builder::UnkEdgeBuilder;
use punc_edge_builder::create_punc_edge_builder;
use latin_edge_builder::create_latin_edge_builder;
use edge::{Edge, EdgeType};
use dict::Dict;

pub struct GraphBuilder<'a> {
    pub path: &'a mut Vec<Edge>,
    pub text: &'a Vec<char>,
    pub dict: &'a Dict,
}

impl<'a> GraphBuilder<'a> {
    pub fn new(text: &'a Vec<char>, path: &'a mut Vec<Edge>, dict: &'a Dict) -> GraphBuilder<'a> {
        path.push(Edge {
            w: 0,
            unk: 0,
            p: 0,
            etype: EdgeType::Init,
        });
        GraphBuilder {
            path: path,
            text: text,
            dict: dict,
        }
    }

    pub fn build(&mut self) {
        let mut builders: Vec<Box<EdgeBuilder>> = vec![Box::new(DictEdgeBuilder::new(self.dict)),
                                                       Box::new(create_latin_edge_builder()),
                                                       Box::new(create_punc_edge_builder()),
                                                       Box::new(UnkEdgeBuilder::new())];
        let mut context = EdgeBuildingContext {
            text: &self.text,
            i: 0,
            ch: '\0',
            left_boundary: 0,
            best_edge: None,
        };
        for i in 0..self.text.len() {
            println!("----------------------------");
            context.ch = self.text[i];
            context.i = i;
            context.best_edge = None;
            for builder in &mut builders {
                let edge = builder.build(&context, &self.path);
                println!("!!! i={} ch={} LEFT={} EDGE = {:?} BEST={:?}",
                         i,
                         context.ch,
                         context.left_boundary,
                         edge,
                         context.best_edge);
                if Edge::better(&edge, &context.best_edge) {
                    println!("@@@ EDGE = {:?}", edge);
                    context.best_edge = edge
                }
            }

            if context.best_edge.is_none() {
                panic!("Best edge cannot be None")
            }

            self.path.push(context.best_edge.unwrap());

            if !context.best_edge.unwrap().is_unk() {
                context.left_boundary = i + 1
            }
        }
    }
}
