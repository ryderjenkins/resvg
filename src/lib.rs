// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

/*!
*resvg* is an [SVG](https://en.wikipedia.org/wiki/Scalable_Vector_Graphics) rendering library.

*resvg* can be used to render SVG files based on a
[static](http://www.w3.org/TR/SVG11/feature#SVG-static)
[SVG Full 1.1](https://www.w3.org/TR/SVG/Overview.html) subset.
In simple terms: no animations and scripting.

It can be used as a simple SVG to PNG converted.
And as an embeddable library to paint SVG on an application native canvas.
*/

#![doc(html_root_url = "https://docs.rs/resvg/0.6.1")]

//#![forbid(unsafe_code)]
#![warn(missing_docs)]

pub use usvg;

#[cfg(feature = "cairo-backend")]
pub use cairo;

#[cfg(feature = "qt-backend")]
pub use resvg_qt as qt;

pub use usvg::{
    svgdom,
    Error,
};


#[cfg(feature = "cairo-backend")] pub mod backend_cairo;
#[cfg(feature = "qt-backend")] pub mod backend_qt;

pub mod utils;
mod backend_utils;
mod geom;
mod layers;
mod options;

/// Commonly used types and traits.
pub mod prelude {
    pub use log::warn;
    pub use usvg;
    pub use usvg::prelude::*;
    pub use usvg::{try_opt, try_opt_warn};
    pub use crate::geom::*;
    pub use crate::utils;
    pub use crate::Options;
    pub use crate::Render;
    pub use crate::OutputImage;
}


use std::path;

pub use crate::options::*;
pub use crate::geom::*;

/// Shorthand names for modules.
mod short {
    pub use crate::svgdom::{
        LengthUnit as Unit,
        ElementId as EId,
        AttributeId as AId,
        AttributeValue as AValue,
    };
}


/// A generic interface for image rendering.
///
/// Instead of using backend implementation directly, you can
/// use this trait to write backend-independent code.
pub trait Render {
    /// Renders SVG to image.
    ///
    /// Returns `None` if an image allocation failed.
    fn render_to_image(
        &self,
        tree: &usvg::Tree,
        opt: &Options,
    ) -> Option<Box<OutputImage>>;

    /// Renders SVG node to image.
    ///
    /// Returns `None` if an image allocation failed.
    fn render_node_to_image(
        &self,
        node: &usvg::Node,
        opt: &Options,
    ) -> Option<Box<OutputImage>>;

    /// Calculates node's absolute bounding box.
    ///
    /// Note: this method can be pretty expensive.
    fn calc_node_bbox(
        &self,
        node: &usvg::Node,
        opt: &Options,
    ) -> Option<Rect>;
}

/// A generic interface for output image.
pub trait OutputImage {
    /// Saves rendered image to the selected path.
    fn save(&self, path: &path::Path) -> bool;
}


/// Returns default backend.
///
/// - If both backends are enabled - cairo backend will be returned.
/// - If no backends are enabled - will panic.
/// - Otherwise will return a corresponding backend.
#[allow(unreachable_code)]
pub fn default_backend() -> Box<Render> {
    #[cfg(feature = "cairo-backend")]
    {
        return Box::new(backend_cairo::Backend);
    }

    #[cfg(feature = "qt-backend")]
    {
        return Box::new(backend_qt::Backend);
    }

    unreachable!("at least one backend must be enabled")
}
