// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use crate::render::prelude::*;

pub fn mask(
    node: &usvg::Node,
    mask: &usvg::Mask,
    bbox: Rect,
    canvas: &mut Canvas,
) {
    let mut mask_pixmap = try_opt!(tiny_skia::Pixmap::new(canvas.pixmap.width(), canvas.pixmap.height()));
    {
        let mut mask_canvas = Canvas::from(mask_pixmap.as_mut());
        mask_canvas.transform = canvas.transform;

        let r = if mask.units == usvg::Units::ObjectBoundingBox {
            mask.rect.bbox_transform(bbox)
        } else {
            mask.rect
        };

        let rr = tiny_skia::Rect::from_xywh(
            r.x() as f32,
            r.y() as f32,
            r.width() as f32,
            r.height() as f32,
        );
        if let Some(rr) = rr {
            mask_canvas.set_clip_rect(rr);
        }

        if mask.content_units == usvg::Units::ObjectBoundingBox {
            mask_canvas.apply_transform(usvg::Transform::from_bbox(bbox).to_native());
        }

        crate::render::render_group(node, &mut RenderState::Ok, &mut mask_canvas);
    }

    {
        use rgb::FromSlice;
        image_to_mask(mask_pixmap.width(), mask_pixmap.height(), mask_pixmap.data_mut().as_rgba_mut());
    }

    if let Some(ref id) = mask.mask {
        if let Some(ref mask_node) = node.tree().defs_by_id(id) {
            if let usvg::NodeKind::Mask(ref mask) = *mask_node.borrow() {
                self::mask(mask_node, mask, bbox, canvas);
            }
        }
    }

    let mut paint = tiny_skia::PixmapPaint::default();
    paint.blend_mode = tiny_skia::BlendMode::DestinationIn;

    canvas.pixmap.draw_pixmap(
        0, 0,
        mask_pixmap.as_ref(),
        &paint,
        tiny_skia::Transform::identity(),
        None,
    );
}

/// Converts an image into an alpha mask.
fn image_to_mask(width: u32, height: u32, data: &mut [rgb::RGBA8]) {
    let coeff_r = 0.2125 / 255.0;
    let coeff_g = 0.7154 / 255.0;
    let coeff_b = 0.0721 / 255.0;

    for y in 0..height {
        for x in 0..width {
            let idx = (y * width + x) as usize;
            let pixel = &mut data[idx];

            let r = pixel.r as f64;
            let g = pixel.g as f64;
            let b = pixel.b as f64;

            let luma = r * coeff_r + g * coeff_g + b * coeff_b;

            pixel.r = 0;
            pixel.g = 0;
            pixel.b = 0;
            pixel.a = usvg::utils::f64_bound(0.0, luma * 255.0, 255.0).ceil() as u8;
        }
    }
}
