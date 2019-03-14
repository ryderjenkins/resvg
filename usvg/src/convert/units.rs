// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use tree;
use super::prelude::*;


pub fn convert_length(
    length: Length,
    node: &svgdom::Node,
    aid: AId,
    object_units: tree::Units,
    state: &State,
) -> f64 {
    let dpi = state.opt.dpi;
    let n = length.num;
    match length.unit {
        Unit::None | Unit::Px => n,
        Unit::Em => n * resolve_font_size(node, state),
        Unit::Ex => n * resolve_font_size(node, state) / 2.0,
        Unit::In => n * dpi,
        Unit::Cm => n * dpi / 2.54,
        Unit::Mm => n * dpi / 25.4,
        Unit::Pt => n * dpi / 72.0,
        Unit::Pc => n * dpi / 6.0,
        Unit::Percent => {
            if object_units == tree::Units::ObjectBoundingBox {
                length.num / 100.0
            } else {
                let view_box = state.view_box;

                match aid {
                    AId::X | AId::Cx | AId::Width  => {
                        convert_percent(length, view_box.width)
                    }
                    AId::Y | AId::Cy | AId::Height => {
                        convert_percent(length, view_box.height)
                    }
                    _ => {
                        let vb_len = (
                            view_box.width * view_box.width + view_box.height * view_box.height
                        ).sqrt() / 2.0_f64.sqrt();

                        convert_percent(length, vb_len)
                    }
                }
            }
        }
    }
}

pub fn convert_list(
    node: &svgdom::Node,
    aid: AId,
    state: &State,
) -> Option<Vec<f64>> {
    let attrs = node.attributes();
    let av = attrs.get_value(aid)?;
    if let AValue::LengthList(ref len_list) = av {
        let mut num_list = Vec::with_capacity(len_list.len());
        for length in len_list.iter() {
            num_list.push(convert_length(*length, node, aid, tree::Units::UserSpaceOnUse, state));
        }

        Some(num_list)
    } else {
        None
    }
}

fn convert_percent(length: Length, base: f64) -> f64 {
    base * length.num / 100.0
}

pub fn resolve_font_size(node: &svgdom::Node, state: &State) -> f64 {
    let nodes: Vec<_> = node.ancestors().collect();
    let mut font_size = state.opt.font_size;
    for n in nodes.iter().rev().skip(1) { // skip Root
        match n.attributes().get_value(AId::FontSize) {
            Some(&AValue::Length(length)) => {
                let dpi = state.opt.dpi;
                let n = length.num;
                font_size = match length.unit {
                    Unit::None | Unit::Px => n,
                    Unit::Em => n * font_size,
                    Unit::Ex => n * font_size / 2.0,
                    Unit::In => n * dpi,
                    Unit::Cm => n * dpi / 2.54,
                    Unit::Mm => n * dpi / 25.4,
                    Unit::Pt => n * dpi / 72.0,
                    Unit::Pc => n * dpi / 6.0,
                    Unit::Percent => {
                        // If `font-size` has percent units that it's value
                        // is relative to the parent node `font-size`.
                        length.num * font_size * 0.01
                    }
                }
            }
            Some(&AValue::String(ref name)) => {
                font_size = convert_named_font_size(name, font_size);
            }
            _ => {}
        }
    }

    font_size
}

fn convert_named_font_size(
    name: &str,
    parent_font_size: f64,
) -> f64 {
    let factor = match name {
        "xx-small"  => -3,
        "x-small"   => -2,
        "small"     => -1,
        "medium"    => 0,
        "large"     => 1,
        "x-large"   => 2,
        "xx-large"  => 3,
        "smaller"   => -1,
        "larger"    => 1,
        _ => {
            warn!("Invalid 'font-size' value: '{}'.", name);
            0
        }
    };

    // 'On a computer screen a scaling factor of 1.2 is suggested between adjacent indexes.'
    parent_font_size * 1.2f64.powi(factor)
}
