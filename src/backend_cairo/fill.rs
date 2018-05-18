// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

// external
use cairo;
use usvg;

// self
use super::prelude::*;
use super::{
    gradient,
    pattern,
};


pub fn apply(
    tree: &usvg::Tree,
    fill: &Option<usvg::Fill>,
    opt: &Options,
    bbox: Rect,
    cr: &cairo::Context,
) {
    match *fill {
        Some(ref fill) => {
            match fill.paint {
                usvg::Paint::Color(c) => {
                    // a-fill-opacity-001.svg
                    cr.set_source_color(&c, fill.opacity);
                }
                usvg::Paint::Link(ref id) => {
                    // a-fill-opacity-003.svg
                    // a-fill-opacity-004.svg
                    if let Some(node) = tree.defs_by_id(id) {
                        match *node.borrow() {
                            usvg::NodeKind::LinearGradient(ref lg) => {
                                gradient::prepare_linear(&node, lg, fill.opacity, bbox, cr);
                            }
                            usvg::NodeKind::RadialGradient(ref rg) => {
                                gradient::prepare_radial(&node, rg, fill.opacity, bbox, cr);
                            }
                            usvg::NodeKind::Pattern(ref pattern) => {
                                pattern::apply(&node, pattern, opt, fill.opacity, bbox, cr);
                            }
                            _ => {}
                        }
                    }
                }
            }

            // a-fill-rule-001.svg
            // a-fill-rule-002.svg
            match fill.rule {
                usvg::FillRule::NonZero => cr.set_fill_rule(cairo::FillRule::Winding),
                usvg::FillRule::EvenOdd => cr.set_fill_rule(cairo::FillRule::EvenOdd),
            }
        }
        None => {
            // reset fill properties
            cr.reset_source_rgba();
            cr.set_fill_rule(cairo::FillRule::Winding);
        }
    }
}
