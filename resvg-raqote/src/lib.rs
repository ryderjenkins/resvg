// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

#![doc(html_root_url = "https://docs.rs/resvg-raqote/0.9.1")]

/// Unwraps `Option` and invokes `return` on `None`.
macro_rules! try_opt {
    ($task:expr) => {
        match $task {
            Some(v) => v,
            None => return,
        }
    };
}

/// Unwraps `Option` and invokes `return $ret` on `None`.
macro_rules! try_opt_or {
    ($task:expr, $ret:expr) => {
        match $task {
            Some(v) => v,
            None => return $ret,
        }
    };
}


use usvg::NodeExt;
use log::warn;

mod clip;
mod filter;
mod image;
mod layers;
mod mask;
mod paint_server;
mod path;
mod render;


/// Rendering options.
pub struct Options {
    /// `usvg` preprocessor options.
    pub usvg: usvg::Options,

    /// Fits the image using specified options.
    ///
    /// Does not affect rendering to canvas.
    pub fit_to: usvg::FitTo,

    /// An image background color.
    ///
    /// Sets an image background color. Does not affect rendering to canvas.
    ///
    /// `None` equals to transparent.
    pub background: Option<usvg::Color>,
}

impl Default for Options {
    fn default() -> Options {
        Options {
            usvg: usvg::Options::default(),
            fit_to: usvg::FitTo::Original,
            background: None,
        }
    }
}


/// Renders SVG to image.
pub fn render_to_image(
    tree: &usvg::Tree,
    opt: &Options,
) -> Option<raqote::DrawTarget> {
    let (mut dt, img_view) = render::create_target(
        tree.svg_node().size.to_screen_size(),
        opt,
    )?;

    // Fill background.
    if let Some(c) = opt.background {
        dt.clear(raqote::SolidSource { r: c.red, g: c.green, b: c.blue, a: 255 });
    }

    render::render_to_canvas(tree, opt, img_view, &mut dt);

    Some(dt)
}

/// Renders SVG node to image.
pub fn render_node_to_image(
    node: &usvg::Node,
    opt: &Options,
) -> Option<raqote::DrawTarget> {
    let node_bbox = if let Some(bbox) = node.calculate_bbox() {
        bbox
    } else {
        warn!("Node '{}' has a zero size.", node.id());
        return None;
    };

    let (mut dt, img_size) = render::create_target(node_bbox.to_screen_size(), opt)?;

    let vbox = usvg::ViewBox {
        rect: node_bbox,
        aspect: usvg::AspectRatio::default(),
    };

    // Fill background.
    if let Some(c) = opt.background {
        dt.clear(raqote::SolidSource { r: c.red, g: c.green, b: c.blue, a: 255 });
    }

    render::render_node_to_canvas(node, opt, vbox, img_size, &mut dt);

    Some(dt)
}
