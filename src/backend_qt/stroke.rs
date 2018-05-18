// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

// external
use qt;
use usvg;

// self
use super::prelude::*;
use super::{
    gradient,
    pattern,
};


pub fn apply(
    tree: &usvg::Tree,
    stroke: &Option<usvg::Stroke>,
    opt: &Options,
    bbox: Rect,
    p: &qt::Painter,
) {
    match *stroke {
        Some(ref stroke) => {
            let mut pen = qt::Pen::new();
            let opacity = stroke.opacity;

            match stroke.paint {
                usvg::Paint::Color(c) => {
                    let a = f64_bound(0.0, *opacity * 255.0, 255.0) as u8;
                    pen.set_color(c.red, c.green, c.blue, a);
                }
                usvg::Paint::Link(ref id) => {
                    // a-stroke-002.svg
                    // a-stroke-003.svg
                    // a-stroke-004.svg
                    let mut brush = qt::Brush::new();

                    if let Some(node) = tree.defs_by_id(id) {
                        match *node.borrow() {
                            usvg::NodeKind::LinearGradient(ref lg) => {
                                gradient::prepare_linear(&node, lg, opacity, bbox, &mut brush);
                            }
                            usvg::NodeKind::RadialGradient(ref rg) => {
                                gradient::prepare_radial(&node, rg, opacity, bbox, &mut brush);
                            }
                            usvg::NodeKind::Pattern(ref pattern) => {
                                let ts = p.get_transform();
                                pattern::apply(&node, pattern, opt, ts, bbox, opacity, &mut brush);
                            }
                            _ => {}
                        }
                    }

                    pen.set_brush(brush);
                }
            }

            // a-stroke-linecap-001.svg
            // a-stroke-linecap-002.svg
            // a-stroke-linecap-003.svg
            let linecap = match stroke.linecap {
                usvg::LineCap::Butt => qt::LineCap::FlatCap,
                usvg::LineCap::Round => qt::LineCap::RoundCap,
                usvg::LineCap::Square => qt::LineCap::SquareCap,
            };
            pen.set_line_cap(linecap);

            // a-stroke-linejoin-001.svg
            // a-stroke-linejoin-002.svg
            // a-stroke-linejoin-003.svg
            let linejoin = match stroke.linejoin {
                usvg::LineJoin::Miter => qt::LineJoin::MiterJoin,
                usvg::LineJoin::Round => qt::LineJoin::RoundJoin,
                usvg::LineJoin::Bevel => qt::LineJoin::BevelJoin,
            };
            pen.set_line_join(linejoin);

            // a-stroke-miterlimit-002.svg
            pen.set_miter_limit(stroke.miterlimit);
            pen.set_width(stroke.width);

            // a-stroke-dasharray-001.svg
            // a-stroke-dasharray-002.svg
            // a-stroke-dashoffset-001.svg
            // a-stroke-dashoffset-002.svg
            // a-stroke-dashoffset-006.svg
            if let Some(ref list) = stroke.dasharray {
                pen.set_dash_offset(stroke.dashoffset);
                pen.set_dash_array(list);
            }

            p.set_pen(pen);
        }
        None => {
            // a-stroke-006.svg
            p.reset_pen();
        }
    }
}
