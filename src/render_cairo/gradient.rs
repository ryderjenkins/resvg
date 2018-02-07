// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

// external
use cairo::{
    self,
    MatrixTrait,
};

// self
use tree;
use math::*;
use traits::{
    ConvTransform,
    TransformFromBBox,
};


pub fn prepare_linear(
    node: tree::DefsNodeRef,
    g: &tree::LinearGradient,
    opacity: f64,
    bbox: Rect,
    cr: &cairo::Context,
) {
    let grad = cairo::LinearGradient::new(g.x1, g.y1, g.x2, g.y2);
    prepare_base(node, &g.d, &grad, opacity, bbox);
    cr.set_source(&grad);
}

pub fn prepare_radial(
    node: tree::DefsNodeRef,
    g: &tree::RadialGradient,
    opacity: f64,
    bbox: Rect,
    cr: &cairo::Context
) {
    let grad = cairo::RadialGradient::new(g.fx, g.fy, 0.0, g.cx, g.cy, g.r);
    prepare_base(node, &g.d, &grad, opacity, bbox);
    cr.set_source(&grad);
}

fn prepare_base(
    node: tree::DefsNodeRef,
    g: &tree::BaseGradient,
    grad: &cairo::Gradient,
    opacity: f64,
    bbox: Rect,
) {
    let spread_method = match g.spread_method {
        tree::SpreadMethod::Pad => cairo::Extend::Pad,
        tree::SpreadMethod::Reflect => cairo::Extend::Reflect,
        tree::SpreadMethod::Repeat => cairo::Extend::Repeat,
    };
    grad.set_extend(spread_method);

    let mut matrix = g.transform.to_native();

    if g.units == tree::Units::ObjectBoundingBox {
        let m = cairo::Matrix::from_bbox(bbox);
        matrix = cairo::Matrix::multiply(&matrix, &m);
    }

    matrix.invert();
    grad.set_matrix(matrix);

    for stop in node.stops() {
        grad.add_color_stop_rgba(
            stop.offset,
            stop.color.red as f64 / 255.0,
            stop.color.green as f64 / 255.0,
            stop.color.blue as f64 / 255.0,
            stop.opacity * opacity,
        );
    }
}
