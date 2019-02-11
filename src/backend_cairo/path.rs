// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

// external
use cairo;

// self
use super::prelude::*;
use super::{
    fill,
    stroke,
};


pub fn draw(
    tree: &usvg::Tree,
    path: &usvg::Path,
    opt: &Options,
    cr: &cairo::Context,
) -> Rect {
    let mut is_square_cap = false;
    if let Some(ref stroke) = path.stroke {
        is_square_cap = stroke.linecap == usvg::LineCap::Square;
    }

    draw_path(&path.segments, is_square_cap, cr);

    let bbox = utils::path_bbox(&path.segments, None, &usvg::Transform::default());

    if path.visibility != usvg::Visibility::Visible {
        return bbox;
    }

    fill::apply(tree, &path.fill, opt, bbox, cr);
    if path.stroke.is_some() {
        cr.fill_preserve();

        stroke::apply(tree, &path.stroke, opt, bbox, cr);
        cr.stroke();
    } else {
        cr.fill();
    }

    bbox
}

fn draw_path(
    segments: &[usvg::PathSegment],
    is_square_cap: bool,
    cr: &cairo::Context,
) {
    // Reset path, in case something was left from the previous paint pass.
    cr.new_path();

    let mut i = 0;
    loop {
        let subpath = get_subpath(i, segments);
        if subpath.is_empty() {
            break;
        }

        draw_subpath(subpath, is_square_cap, cr);
        i += subpath.len();
    }
}

fn get_subpath(start: usize, segments: &[usvg::PathSegment]) -> &[usvg::PathSegment] {
    let mut i = start;
    while i < segments.len() {
        match segments[i] {
            usvg::PathSegment::MoveTo { .. } => {
                if i != start {
                    break;
                }
            }
            usvg::PathSegment::ClosePath => {
                i += 1;
                break;
            }
            _ => {}
        }

        i += 1;
    }

    &segments[start..i]
}

fn draw_subpath(
    segments: &[usvg::PathSegment],
    is_square_cap: bool,
    cr: &cairo::Context,
) {
    assert_ne!(segments.len(), 0);

    // This is a workaround for a cairo bug(?).
    //
    // Buy the SVG spec, a zero length subpath with a square cap should be
    // rendered as a square/rect, but it's not (at least on 1.14.12/1.15.12).
    // And this is probably a bug, since round caps are rendered correctly.
    let mut is_zero_path = false;
    if is_square_cap {
        if utils::path_length(segments).is_fuzzy_zero() {
            is_zero_path = true;
        }
    }

    if !is_zero_path {
        for seg in segments {
            match *seg {
                usvg::PathSegment::MoveTo { x, y } => {
                    cr.new_sub_path();
                    cr.move_to(x, y);
                }
                usvg::PathSegment::LineTo { x, y } => {
                    cr.line_to(x, y);
                }
                usvg::PathSegment::CurveTo { x1, y1, x2, y2, x, y } => {
                    cr.curve_to(x1, y1, x2, y2, x, y);
                }
                usvg::PathSegment::ClosePath => {
                    cr.close_path();
                }
            }
        }
    } else {
        if let usvg::PathSegment::MoveTo { x, y } = segments[0] {
            // Draw zero length path.
            let shift = 0.002; // Purely empirical.
            cr.new_sub_path();
            cr.move_to(x, y);
            cr.line_to(x + shift, y);
        }
    }
}
