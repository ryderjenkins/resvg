// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

// external
use cairo::{
    self,
    MatrixTrait,
    Pattern,
};
use usvg;
use usvg::prelude::*;

// self
use super::prelude::*;


pub fn apply(
    node: &usvg::Node,
    pattern: &usvg::Pattern,
    opt: &Options,
    opacity: usvg::Opacity,
    bbox: Rect,
    cr: &cairo::Context,
) {
    let r = if pattern.units == usvg::Units::ObjectBoundingBox {
        pattern.rect.transform(usvg::Transform::from_bbox(bbox))
    } else {
        pattern.rect
    };

    let global_ts = usvg::Transform::from_native(&cr.get_matrix());
    let (sx, sy) = global_ts.get_scale();

    let img_size = Size::new(r.width * sx, r.height * sy).to_screen_size();
    let surface = try_create_surface!(img_size, ());

    let sub_cr = cairo::Context::new(&surface);
    sub_cr.transform(cairo::Matrix::new(sx, 0.0, 0.0, sy, 0.0, 0.0));

    if let Some(vbox) = pattern.view_box {
        let ts = utils::view_box_to_transform(vbox.rect, vbox.aspect, r.size());
        sub_cr.transform(ts.to_native());
    } else if pattern.content_units == usvg::Units::ObjectBoundingBox {
        // 'Note that this attribute has no effect if attribute `viewBox` is specified.'

        // We don't use Transform::from_bbox(bbox) because `x` and `y` should be
        // ignored for some reasons...
        sub_cr.scale(bbox.width, bbox.height);
    }

    let mut layers = super::create_layers(img_size, opt);
    super::render_group(node, opt, &mut layers, &sub_cr);

    let mut ts = usvg::Transform::default();
    ts.append(&pattern.transform);
    ts.translate(r.x, r.y);
    ts.scale(1.0 / sx, 1.0 / sy);


    let surface = if opacity.fuzzy_ne(&1.0) {
        // If `opacity` isn't `1` then we have to make image semitransparent.
        // The only way to do this is by making a new image and rendering
        // the pattern on it with transparency.

        let surface2 = try_create_surface!(img_size, ());
        let sub_cr2 = cairo::Context::new(&surface2);
        sub_cr2.set_source_surface(&surface, 0.0, 0.0);
        sub_cr2.paint_with_alpha(*opacity);

        surface2
    } else {
        surface
    };


    let patt = cairo::SurfacePattern::create(&surface);
    patt.set_extend(cairo::Extend::Repeat);
    patt.set_filter(cairo::Filter::Best);

    let mut m: cairo::Matrix = ts.to_native();
    m.invert();
    patt.set_matrix(m);

    cr.set_source(&patt);
}
