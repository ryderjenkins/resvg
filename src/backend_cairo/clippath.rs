// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

// external
use cairo::{
    self,
    MatrixTrait,
};
use usvg;
use usvg::prelude::*;

// self
use super::prelude::*;
use super::{
    path,
    text,
};


pub fn apply(
    node: &usvg::Node,
    cp: &usvg::ClipPath,
    opt: &Options,
    bbox: Rect,
    layers: &mut CairoLayers,
    cr: &cairo::Context,
) {
    let clip_surface = try_opt!(layers.get(), ());
    let clip_surface = clip_surface.borrow_mut();

    let clip_cr = cairo::Context::new(&*clip_surface);
    clip_cr.set_source_rgba(0.0, 0.0, 0.0, 1.0);
    clip_cr.paint();
    clip_cr.set_matrix(cr.get_matrix());
    clip_cr.transform(cp.transform.to_native());

    if cp.units == usvg::Units::ObjectBoundingBox {
        let m = cairo::Matrix::from_bbox(bbox);
        clip_cr.transform(m);
    }

    clip_cr.set_operator(cairo::Operator::Clear);

    let matrix = clip_cr.get_matrix();
    for node in node.children() {
        clip_cr.transform(node.transform().to_native());

        match *node.borrow() {
            usvg::NodeKind::Path(ref p) => {
                path::draw(&node.tree(), p, opt, &clip_cr);
            }
            usvg::NodeKind::Text(_) => {
                text::draw(&node, opt, &clip_cr);
            }
            _ => {}
        }

        clip_cr.set_matrix(matrix);
    }

    cr.set_matrix(cairo::Matrix::identity());
    cr.set_source_surface(&*clip_surface, 0.0, 0.0);
    cr.set_operator(cairo::Operator::DestOut);
    cr.paint();

    // Reset operator.
    cr.set_operator(cairo::Operator::Over);

    // Reset source to unborrow the `clip_surface` from the `Context`.
    cr.reset_source_rgba();
}
