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

use edge::{Edge,EdgeType};
use edge_builder::{EdgeBuilder, EdgeBuildingContext};

pub struct UnkEdgeBuilder {
}

impl UnkEdgeBuilder {
    pub fn new() -> UnkEdgeBuilder {
        UnkEdgeBuilder{}
    }
}

impl EdgeBuilder for UnkEdgeBuilder {
    fn build(&mut self, context: &EdgeBuildingContext, path: &[Edge]) -> Option<Edge> {
        if context.best_edge.is_some() {
            return None
        }

        let source = path[context.left_boundary];
        Some(Edge{p: context.left_boundary,
                  etype: EdgeType::Unk,
                  unk: source.unk + 1,
                  w: source.w + 1})
    }
}

