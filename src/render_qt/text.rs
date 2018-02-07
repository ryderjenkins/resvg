// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use std::f64;

// external
use qt;

// self
use tree;
use render_utils;
use math::*;
use super::{
    fill,
    stroke,
};


pub fn draw(
    rtree: &tree::RenderTree,
    node: tree::NodeRef,
    p: &qt::Painter,
) -> Rect {
    draw_tspan(node, p,
        |tspan, x, y, w, font| _draw_tspan(rtree, tspan, x, y, w, &font, p))
}

pub fn draw_tspan<DrawAt>(
    node: tree::NodeRef,
    p: &qt::Painter,
    mut draw_at: DrawAt
) -> Rect
    where DrawAt: FnMut(&tree::TSpan, f64, f64, f64, &qt::Font)
{
    let mut bbox = Rect::from_xywh(f64::MAX, f64::MAX, 0.0, 0.0);
    let mut font_list = Vec::new();
    let mut tspan_w_list = Vec::new();
    for (child, chunk) in node.text_chunks() {
        font_list.clear();
        tspan_w_list.clear();
        let mut chunk_width = 0.0;

        for tspan in child.text_spans() {
            let font = init_font(&tspan.font);
            p.set_font(&font);
            let font_metrics = p.font_metrics();
            let tspan_width = font_metrics.width(&tspan.text);

            font_list.push(font);
            chunk_width += tspan_width;
            tspan_w_list.push(tspan_width);

            bbox.expand(chunk.x, chunk.y - font_metrics.ascent(),
                        chunk_width, font_metrics.height());
        }

        let mut x = render_utils::process_text_anchor(chunk.x, chunk.anchor, chunk_width);

        for ((tspan, width), font) in child.text_spans().zip(&tspan_w_list).zip(&font_list) {
            draw_at(tspan, x, chunk.y, *width, &font);
            x += width;
        }
    }

    bbox
}

fn _draw_tspan(
    rtree: &tree::RenderTree,
    tspan: &tree::TSpan,
    x: f64,
    mut y: f64,
    width: f64,
    font: &qt::Font,
    p: &qt::Painter,
) {
    p.set_font(&font);
    let font_metrics = p.font_metrics();

    let baseline_offset = font_metrics.ascent();
    y -= baseline_offset;

    let mut line_rect = Rect::from_xywh(
        x,
        0.0,
        width,
        font_metrics.line_width(),
    );

    // Draw underline.
    //
    // Should be drawn before/under text.
    if let Some(ref style) = tspan.decoration.underline {
        line_rect.origin.y = y + font_metrics.height() - font_metrics.underline_pos();
        draw_line(rtree, &style.fill, &style.stroke, line_rect, p);
    }

    // Draw overline.
    //
    // Should be drawn before/under text.
    if let Some(ref style) = tspan.decoration.overline {
        line_rect.origin.y = y + font_metrics.height() - font_metrics.overline_pos();
        draw_line(rtree, &style.fill, &style.stroke, line_rect, p);
    }

    let bbox = Rect::from_xywh(0.0, 0.0, width, font_metrics.height());

    // Draw text.
    fill::apply(rtree, &tspan.fill, p, bbox);
    stroke::apply(rtree, &tspan.stroke, p, bbox);

    p.draw_text(x, y, &tspan.text);

    // Draw line-through.
    //
    // Should be drawn after/over text.
    if let Some(ref style) = tspan.decoration.line_through {
        line_rect.origin.y = y + baseline_offset - font_metrics.strikeout_pos();
        draw_line(rtree, &style.fill, &style.stroke, line_rect, p);
    }
}

pub fn init_font(dom_font: &tree::Font) -> qt::Font {
    let mut font = qt::Font::new();

    font.set_family(&dom_font.family);

    let font_style = match dom_font.style {
        tree::FontStyle::Normal => qt::FontStyle::StyleNormal,
        tree::FontStyle::Italic => qt::FontStyle::StyleItalic,
        tree::FontStyle::Oblique => qt::FontStyle::StyleOblique,
    };
    font.set_style(font_style);

    if dom_font.variant == tree::FontVariant::SmallCaps {
        font.set_small_caps(true);
    }

    let font_weight = match dom_font.weight {
        tree::FontWeight::W100       => qt::FontWeight::Thin,
        tree::FontWeight::W200       => qt::FontWeight::ExtraLight,
        tree::FontWeight::W300       => qt::FontWeight::Light,
        tree::FontWeight::W400       => qt::FontWeight::Normal,
        tree::FontWeight::W500       => qt::FontWeight::Medium,
        tree::FontWeight::W600       => qt::FontWeight::DemiBold,
        tree::FontWeight::W700       => qt::FontWeight::Bold,
        tree::FontWeight::W800       => qt::FontWeight::ExtraBold,
        tree::FontWeight::W900       => qt::FontWeight::Black,
        tree::FontWeight::Normal     => qt::FontWeight::Normal,
        tree::FontWeight::Bold       => qt::FontWeight::Bold,
        tree::FontWeight::Bolder     => qt::FontWeight::ExtraBold,
        tree::FontWeight::Lighter    => qt::FontWeight::Light,
    };
    font.set_weight(font_weight);

    let font_stretch = match dom_font.stretch {
        tree::FontStretch::Normal         => qt::FontStretch::Unstretched,
        tree::FontStretch::Narrower |
        tree::FontStretch::Condensed      => qt::FontStretch::Condensed,
        tree::FontStretch::UltraCondensed => qt::FontStretch::UltraCondensed,
        tree::FontStretch::ExtraCondensed => qt::FontStretch::ExtraCondensed,
        tree::FontStretch::SemiCondensed  => qt::FontStretch::SemiCondensed,
        tree::FontStretch::SemiExpanded   => qt::FontStretch::SemiExpanded,
        tree::FontStretch::Wider |
        tree::FontStretch::Expanded       => qt::FontStretch::Expanded,
        tree::FontStretch::ExtraExpanded  => qt::FontStretch::ExtraExpanded,
        tree::FontStretch::UltraExpanded  => qt::FontStretch::UltraExpanded,
    };
    font.set_stretch(font_stretch);

    font.set_size(dom_font.size);

    font
}

fn draw_line(
    rtree: &tree::RenderTree,
    fill: &Option<tree::Fill>,
    stroke: &Option<tree::Stroke>,
    line_bbox: Rect,
    p: &qt::Painter,
) {
    let mut p_path = qt::PainterPath::new();

    p_path.move_to(line_bbox.x(),  line_bbox.y());
    p_path.line_to(line_bbox.x() + line_bbox.width(),  line_bbox.y());
    p_path.line_to(line_bbox.x() + line_bbox.width(),  line_bbox.y() + line_bbox.height());
    p_path.line_to(line_bbox.x(),  line_bbox.y() + line_bbox.height());
    p_path.close_path();

    fill::apply(rtree, fill, p, line_bbox);
    stroke::apply(rtree, stroke, p, line_bbox);

    p.draw_path(&p_path);
}
