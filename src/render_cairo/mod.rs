// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

//! Cairo backend implementation.

use std::f64;

// external
use cairo::{
    self,
    MatrixTrait,
};
use pangocairo::functions as pc;

// self
use tree::{
    self,
    NodeExt,
};
use math::*;
use traits::{
    ConvTransform,
    TransformFromBBox,
};
use {
    ErrorKind,
    Options,
    Result,
    Render,
    OutputImage,
};
use utils;
use self::ext::*;


mod clippath;
mod ext;
mod fill;
mod gradient;
mod image;
mod path;
mod pattern;
mod stroke;
mod text;


impl ConvTransform<cairo::Matrix> for tree::Transform {
    fn to_native(&self) -> cairo::Matrix {
        cairo::Matrix::new(self.a, self.b, self.c, self.d, self.e, self.f)
    }

    fn from_native(ts: &cairo::Matrix) -> Self {
        Self::new(ts.xx, ts.yx, ts.xy, ts.yy, ts.x0, ts.y0)
    }
}

impl TransformFromBBox for cairo::Matrix {
    fn from_bbox(bbox: Rect) -> Self {
        Self::new(bbox.width(), 0.0, 0.0, bbox.height(), bbox.x(), bbox.y())
    }
}

/// Cairo backend handle.
pub struct Backend;

impl Render for Backend {
    fn render_to_image(
        &self,
        rtree: &tree::RenderTree,
        opt: &Options,
    ) -> Result<Box<OutputImage>> {
        let img = render_to_image(rtree, opt)?;
        Ok(Box::new(img))
    }

    fn render_node_to_image(
        &self,
        rtree: &tree::RenderTree,
        node: tree::NodeRef,
        opt: &Options,
    ) -> Result<Box<OutputImage>> {
        let img = render_node_to_image(rtree, node, opt)?;
        Ok(Box::new(img))
    }

    fn calc_node_bbox(
        &self,
        rtree: &tree::RenderTree,
        node: tree::NodeRef,
        opt: &Options,
    ) -> Option<Rect> {
        calc_node_bbox(rtree, node, opt)
    }
}

impl OutputImage for cairo::ImageSurface {
    fn save(&self, path: &::std::path::Path) -> bool {
        use std::fs;

        if let Ok(mut buffer) = fs::File::create(path) {
            if let Ok(_) = self.write_to_png(&mut buffer) {
                return true;
            }
        }

        false
    }
}


/// Renders SVG to image.
pub fn render_to_image(
    rtree: &tree::RenderTree,
    opt: &Options,
) -> Result<cairo::ImageSurface> {
    let (surface, img_view) = create_surface(rtree.svg_node().size, opt)?;

    let cr = cairo::Context::new(&surface);

    // Fill background.
    if let Some(color) = opt.background {
        cr.set_source_color(&color, 1.0);
        cr.paint();
    }

    render_to_canvas(&cr, img_view, rtree);

    Ok(surface)
}

/// Renders SVG to image.
pub fn render_node_to_image(
    rtree: &tree::RenderTree,
    node: tree::NodeRef,
    opt: &Options,
) -> Result<cairo::ImageSurface> {
    let node_bbox = if let Some(bbox) = calc_node_bbox(rtree, node, opt) {
        bbox
    } else {
        warn!("Node {:?} has zero size.", node.svg_id());
        return Err(ErrorKind::NoCanvas.into());
    };

    let (surface, img_view) = create_surface(node_bbox.size, opt)?;

    let cr = cairo::Context::new(&surface);

    // Fill background.
    if let Some(color) = opt.background {
        cr.set_source_color(&color, 1.0);
        cr.paint();
    }

    apply_viewbox_transform(&cr, node_bbox, img_view);
    render_node_to_canvas(&cr, img_view, rtree, node);

    Ok(surface)
}

/// Renders SVG to canvas.
pub fn render_to_canvas(
    cr: &cairo::Context,
    img_view: Rect,
    rtree: &tree::RenderTree,
) {
    apply_viewbox_transform(cr, rtree.svg_node().view_box, img_view);
    render_group(rtree, rtree.root(), &cr, &cr.get_matrix(), img_view.size);
}

/// Renders SVG node to canvas.
pub fn render_node_to_canvas(
    cr: &cairo::Context,
    img_view: Rect,
    rtree: &tree::RenderTree,
    node: tree::NodeRef,
) {
    let curr_ts = cr.get_matrix();
    let mut ts = utils::abs_transform(node);
    ts.append(&node.transform());

    cr.transform(ts.to_native());
    render_node(rtree, node, cr, img_view.size);
    cr.set_matrix(curr_ts);
}

fn create_surface(
    size: Size,
    opt: &Options,
) -> Result<(cairo::ImageSurface, Rect)> {
    let img_size = utils::fit_to(size, opt.fit_to);

    debug_assert!(!img_size.is_empty_or_negative());

    let surface = cairo::ImageSurface::create(
        cairo::Format::ARgb32,
        img_size.width as i32,
        img_size.height as i32
    );

    let surface = match surface {
        Ok(v) => v,
        Err(_) => {
            return Err(ErrorKind::NoCanvas.into());
        }
    };

    let img_view = Rect::new(Point::new(0.0, 0.0), img_size);

    Ok((surface, img_view))
}

/// Applies viewbox transformation to the painter.
fn apply_viewbox_transform(
    cr: &cairo::Context,
    view_box: Rect,
    img_view: Rect,
) {
    let ts = {
        let (dx, dy, sx, sy) = utils::view_box_transform(view_box, img_view);
        cairo::Matrix::new(sx, 0.0, 0.0, sy, dx, dy)
    };
    cr.transform(ts);
}

fn render_group(
    rtree: &tree::RenderTree,
    node: tree::NodeRef,
    cr: &cairo::Context,
    matrix: &cairo::Matrix,
    img_size: Size,
) -> Rect {
    let mut g_bbox = Rect::from_xywh(f64::MAX, f64::MAX, 0.0, 0.0);

    for node in node.children() {
        cr.transform(node.transform().to_native());

        let bbox = render_node(rtree, node, cr, img_size);

        if let Some(bbox) = bbox {
            g_bbox.expand_from_rect(bbox);
        }

        cr.set_matrix(*matrix);
    }

    g_bbox
}

fn render_group_impl(
    rtree: &tree::RenderTree,
    node: tree::NodeRef,
    g: &tree::Group,
    cr: &cairo::Context,
    img_size: Size,
) -> Option<Rect> {
    let sub_surface = cairo::ImageSurface::create(
        cairo::Format::ARgb32,
        img_size.width as i32,
        img_size.height as i32
    );

    let sub_surface = match sub_surface {
        Ok(surf) => surf,
        Err(_) => {
            warn!("Subsurface creation failed.");
            return None;
        }
    };

    let sub_cr = cairo::Context::new(&sub_surface);
    sub_cr.set_matrix(cr.get_matrix());

    let bbox = render_group(rtree, node, &sub_cr, &cr.get_matrix(), img_size);

    if let Some(idx) = g.clip_path {
        let clip_node = rtree.defs_at(idx);
        if let tree::NodeKind::ClipPath(ref cp) = *clip_node.value() {
            clippath::apply(rtree, clip_node, cp, &sub_cr, bbox, img_size);
        }
    }

    let curr_matrix = cr.get_matrix();
    cr.set_matrix(cairo::Matrix::identity());

    cr.set_source_surface(&sub_surface, 0.0, 0.0);

    if let Some(opacity) = g.opacity {
        cr.paint_with_alpha(opacity);
    } else {
        cr.paint();
    }

    cr.set_matrix(curr_matrix);

    Some(bbox)
}

fn render_node(
    rtree: &tree::RenderTree,
    node: tree::NodeRef,
    cr: &cairo::Context,
    img_size: Size,
) -> Option<Rect> {
    match *node.value() {
        tree::NodeKind::Path(ref path) => {
            Some(path::draw(rtree, path, cr))
        }
        tree::NodeKind::Text(_) => {
            Some(text::draw(rtree, node, cr))
        }
        tree::NodeKind::Image(ref img) => {
            Some(image::draw(img, cr))
        }
        tree::NodeKind::Group(ref g) => {
            render_group_impl(rtree, node, g, cr, img_size)
        }
        _ => None,
    }
}

/// Calculates node's bounding box.
///
/// Note: this method can be pretty expensive.
pub fn calc_node_bbox(
    rtree: &tree::RenderTree,
    node: tree::NodeRef,
    opt: &Options,
) -> Option<Rect> {
    // We can't use 1x1 image, like in Qt backend because otherwise
    // text layouts will be truncated.
    let (surface, img_view) = create_surface(rtree.svg_node().size, opt).unwrap();
    let cr = cairo::Context::new(&surface);

    // We also have to apply the viewbox transform,
    // otherwise text hinting will be different and bbox will be different too.
    apply_viewbox_transform(&cr, rtree.svg_node().view_box, img_view);

    let abs_ts = utils::abs_transform(node);
    _calc_node_bbox(&cr, rtree, node, abs_ts)
}

fn _calc_node_bbox(
    cr: &cairo::Context,
    rtree: &tree::RenderTree,
    node: tree::NodeRef,
    ts: tree::Transform,
) -> Option<Rect> {
    let mut ts2 = ts;
    ts2.append(&node.transform());

    match *node.value() {
        tree::NodeKind::Path(ref path) => {
            Some(utils::path_bbox(&path.segments, &path.stroke, &ts2))
        }
        tree::NodeKind::Text(_) => {
            let mut bbox = Rect::from_xywh(f64::MAX, f64::MAX, 0.0, 0.0);

            text::draw_tspan(rtree, node, cr, |tspan, x, y, _, pd| {
                cr.new_path();

                pc::layout_path(cr, &pd.layout);
                // We are using `copy_path_flat` instead of `copy_path`
                // because the last one produces an invalid bbox.
                let path = cr.copy_path_flat();
                let segments = from_cairo_path(&path);

                let mut t = ts2;
                t.append(&tree::Transform::new(1.0, 0.0, 0.0, 1.0, x, y));

                if !segments.is_empty() {
                    let c_bbox = utils::path_bbox(&segments, &tspan.stroke, &t);
                    bbox.expand_from_rect(c_bbox);
                }
            });

            Some(bbox)
        }
        tree::NodeKind::Image(ref img) => {
            let segments = utils::rect_to_path(img.rect);
            Some(utils::path_bbox(&segments, &None, &ts2))
        }
        tree::NodeKind::Group(_) => {
            let mut bbox = Rect::from_xywh(f64::MAX, f64::MAX, 0.0, 0.0);

            for child in node.children() {
                if let Some(c_bbox) = _calc_node_bbox(cr, rtree, child, ts2) {
                    bbox.expand_from_rect(c_bbox);
                }
            }

            Some(bbox)
        }
        _ => None
    }
}

fn from_cairo_path(path: &cairo::Path) -> Vec<tree::PathSegment> {
    let mut segments = Vec::new();
    for seg in path.iter() {
        match seg {
            cairo::PathSegment::MoveTo((x, y)) => {
                segments.push(tree::PathSegment::MoveTo { x, y });
            }
            cairo::PathSegment::LineTo((x, y)) => {
                segments.push(tree::PathSegment::LineTo { x, y });
            }
            cairo::PathSegment::CurveTo((x1, y1), (x2, y2), (x, y)) => {
                segments.push(tree::PathSegment::CurveTo { x1, y1, x2, y2, x, y });
            }
            cairo::PathSegment::ClosePath => {
                segments.push(tree::PathSegment::ClosePath);
            }
        }
    }

    segments
}
