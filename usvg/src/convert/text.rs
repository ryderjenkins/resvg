// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

// external
use svgdom;

// self
use tree;
use tree::prelude::*;
use super::prelude::*;
use super::{
    fill,
    stroke,
};


pub fn convert(
    text_elem: &svgdom::Node,
    opt: &Options,
    mut parent: tree::Node,
    tree: &mut tree::Tree,
) {
    let chunks = try_opt!(convert_chunks(text_elem, tree, opt), ());
    let attrs = text_elem.attributes();
    parent.append_kind(tree::NodeKind::Text(tree::Text {
        id: text_elem.id().clone(),
        transform: attrs.get_transform(AId::Transform).unwrap_or_default(),
        rotate: attrs.get_number_list(AId::Rotate).cloned(),
        chunks,
    }));
}

fn convert_chunks(
    text_elem: &svgdom::Node,
    tree: &tree::Tree,
    opt: &Options,
) -> Option<Vec<tree::TextChunk>> {
    let mut chunks = Vec::new();

    {
        let ref root_attrs = text_elem.attributes();
        let chunk_node = tree::TextChunk {
            x: root_attrs.get_number_list(AId::X).cloned(),
            y: root_attrs.get_number_list(AId::Y).cloned(),
            dx: root_attrs.get_number_list(AId::Dx).cloned(),
            dy: root_attrs.get_number_list(AId::Dy).cloned(),
            anchor: conv_text_anchor(root_attrs),
            spans: Vec::new(),
        };
        chunks.push(chunk_node);
    }

    for tspan in text_elem.children() {
        debug_assert!(tspan.is_tag_name(EId::Tspan));

        let text = match tspan.first_child() {
            Some(node) => node.text().clone(),
            _ => continue,
        };

        let ref attrs = tspan.attributes();
        let x = attrs.get_number_list(AId::X).cloned();
        let y = attrs.get_number_list(AId::Y).cloned();
        let dx = attrs.get_number_list(AId::Dx).cloned();
        let dy = attrs.get_number_list(AId::Dy).cloned();

        if x.is_some() || y.is_some() || dx.is_some() || dy.is_some() {
            let chunk_node = tree::TextChunk {
                x,
                y,
                dx,
                dy,
                anchor: conv_text_anchor(attrs),
                spans: Vec::new(),
            };
            chunks.push(chunk_node);
        }

        let fill = fill::convert(tree, attrs, true);
        let stroke = stroke::convert(tree, attrs, true);
        let decoration = conv_tspan_decoration2(tree, text_elem, &tspan);
        let visibility = super::convert_visibility(attrs);
        let span = tree::TextSpan {
            visibility,
            fill,
            stroke,
            font: convert_font(attrs, opt),
            decoration,
            text,
        };

        let last_idx = chunks.len() - 1;
        chunks[last_idx].spans.push(span);
    }

    debug_assert!(!chunks.is_empty());

    if !chunks.is_empty() {
        Some(chunks)
    } else {
        None
    }
}

struct TextDecoTypes {
    has_underline: bool,
    has_overline: bool,
    has_line_through: bool,
}

// 'text-decoration' defined in the 'text' element
// should be generated by 'prepare_text_decoration'.
fn conv_text_decoration(node: &svgdom::Node) -> TextDecoTypes {
    debug_assert!(node.is_tag_name(EId::Text));

    let attrs = node.attributes();

    let text = attrs.get_str_or(AId::TextDecoration, "");

    TextDecoTypes {
        has_underline: text.contains("underline"),
        has_overline: text.contains("overline"),
        has_line_through: text.contains("line-through"),
    }
}

// 'text-decoration' in 'tspan' does not depend on parent elements.
fn conv_tspan_decoration(tspan: &svgdom::Node) -> TextDecoTypes {
    debug_assert!(tspan.is_tag_name(EId::Tspan));

    let attrs = tspan.attributes();

    let has_attr = |decoration_id: &str| {
        if let Some(id) = attrs.get_str(AId::TextDecoration) {
            if id == decoration_id {
                return true;
            }
        }

        false
    };

    TextDecoTypes {
        has_underline: has_attr("underline"),
        has_overline: has_attr("overline"),
        has_line_through: has_attr("line-through"),
    }
}

fn conv_tspan_decoration2(
    tree: &tree::Tree,
    node: &svgdom::Node,
    tspan: &svgdom::Node
) -> tree::TextDecoration {
    let text_dec = conv_text_decoration(node);
    let tspan_dec = conv_tspan_decoration(tspan);

    let gen_style = |in_tspan: bool, in_text: bool| {
        let n = if in_tspan {
            tspan.clone()
        } else if in_text {
            node.clone()
        } else {
            return None;
        };

        let ref attrs = n.attributes();
        let fill = fill::convert(tree, attrs, true);
        let stroke = stroke::convert(tree, attrs, true);

        Some(tree::TextDecorationStyle {
            fill,
            stroke,
        })
    };

    tree::TextDecoration {
        underline: gen_style(tspan_dec.has_underline, text_dec.has_underline),
        overline: gen_style(tspan_dec.has_overline, text_dec.has_overline),
        line_through: gen_style(tspan_dec.has_line_through, text_dec.has_line_through),
    }
}

fn conv_text_anchor(attrs: &svgdom::Attributes) -> tree::TextAnchor {
    let av = attrs.get_str_or(AId::TextAnchor, "start");
    match av {
        "start" => tree::TextAnchor::Start,
        "middle" => tree::TextAnchor::Middle,
        "end" => tree::TextAnchor::End,
        _ => tree::TextAnchor::Start,
    }
}

fn convert_font(attrs: &svgdom::Attributes, opt: &Options) -> tree::Font {
    let style = attrs.get_str_or(AId::FontStyle, "normal");
    let style = match style {
        "normal" => tree::FontStyle::Normal,
        "italic" => tree::FontStyle::Italic,
        "oblique" => tree::FontStyle::Oblique,
        _ => tree::FontStyle::Normal,
    };

    let variant = attrs.get_str_or(AId::FontVariant, "normal");
    let variant = match variant {
        "normal" => tree::FontVariant::Normal,
        "small-caps" => tree::FontVariant::SmallCaps,
        _ => tree::FontVariant::Normal,
    };

    let weight = attrs.get_str_or(AId::FontWeight, "normal");
    let weight = match weight {
        "normal" => tree::FontWeight::W400,
        "bold" => tree::FontWeight::W700,
        "100" => tree::FontWeight::W100,
        "200" => tree::FontWeight::W200,
        "300" => tree::FontWeight::W300,
        "400" => tree::FontWeight::W400,
        "500" => tree::FontWeight::W500,
        "600" => tree::FontWeight::W600,
        "700" => tree::FontWeight::W700,
        "800" => tree::FontWeight::W800,
        "900" => tree::FontWeight::W900,
        "bolder" | "lighter" => {
            warn!("'bolder' and 'lighter' font-weight must be already resolved.");
            tree::FontWeight::W400
        }
        _ => tree::FontWeight::W400,
    };

    let stretch = attrs.get_str_or(AId::FontStretch, "normal");
    let stretch = match stretch {
        "normal" => tree::FontStretch::Normal,
        "wider" => tree::FontStretch::Wider,
        "narrower" => tree::FontStretch::Narrower,
        "ultra-condensed" => tree::FontStretch::UltraCondensed,
        "extra-condensed" => tree::FontStretch::ExtraCondensed,
        "condensed" => tree::FontStretch::Condensed,
        "semi-condensed" => tree::FontStretch::SemiCondensed,
        "semi-expanded" => tree::FontStretch::SemiExpanded,
        "expanded" => tree::FontStretch::Expanded,
        "extra-expanded" => tree::FontStretch::ExtraExpanded,
        "ultra-expanded" => tree::FontStretch::UltraExpanded,
        _ => tree::FontStretch::Normal,
    };

    // TODO: what to do when <= 0?
    let size = attrs.get_number_or(AId::FontSize, opt.font_size);
    let size = if !(size > 0.0) { opt.font_size } else { size };
    let size = tree::FontSize::new(size);

    let family = attrs.get_str_or(AId::FontFamily, &opt.font_family).to_owned();

    tree::Font {
        family,
        size,
        style,
        variant,
        weight,
        stretch,
    }
}
