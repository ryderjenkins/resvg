// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use crate::svgtree::{self, AId};
use crate::{FilterInput, FilterKind, FilterPrimitive};

/// A merge filter primitive.
///
/// `feMerge` element in the SVG.
#[derive(Clone, Debug)]
pub struct FeMerge {
    /// List of input layers that should be merged.
    ///
    /// List of `feMergeNode`'s in the SVG.
    pub inputs: Vec<FilterInput>,
}

pub(crate) fn convert(fe: svgtree::Node, primitives: &[FilterPrimitive]) -> FilterKind {
    let mut inputs = Vec::new();
    for child in fe.children() {
        inputs.push(super::resolve_input(child, AId::In, primitives));
    }

    FilterKind::FeMerge(FeMerge {
        inputs,
    })
}
