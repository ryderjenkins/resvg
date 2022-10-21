// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use std::rc::Rc;

use crate::svgtree::{self, EId, AId};
use crate::{converter, NodeKind, Units, Transform, Node, Group};


/// A clip-path element.
///
/// `clipPath` element in SVG.
#[derive(Clone, Debug)]
pub struct ClipPath {
    /// Element's ID.
    ///
    /// Taken from the SVG itself or generated by the parser.
    /// Used only during SVG writing. `resvg` doesn't rely on this property.
    pub id: String,

    /// Coordinate system units.
    ///
    /// `clipPathUnits` in SVG.
    pub units: Units,

    /// Clip path transform.
    ///
    /// `transform` in SVG.
    pub transform: Transform,

    /// Additional clip path.
    ///
    /// `clip-path` in SVG.
    pub clip_path: Option<Rc<Self>>,

    /// Clip path children.
    ///
    /// The root node is always `Group`.
    pub root: Node,
}

impl Default for ClipPath {
    fn default() -> Self {
        ClipPath {
            id: String::new(),
            units: Units::UserSpaceOnUse,
            transform: Transform::default(),
            clip_path: None,
            root: Node::new(NodeKind::Group(Group::default())),
        }
    }
}


pub(crate) fn convert(
    node: svgtree::Node,
    state: &converter::State,
    cache: &mut converter::Cache,
) -> Option<Rc<ClipPath>> {
    // A `clip-path` attribute must reference a `clipPath` element.
    if !node.has_tag_name(EId::ClipPath) {
        return None;
    }

    if !node.has_valid_transform(AId::Transform) {
        return None;
    }

    // Check if this element was already converted.
    if let Some(clip) = cache.clip_paths.get(node.element_id()) {
        return Some(clip.clone());
    }

    // Resolve linked clip path.
    let mut clip_path = None;
    if let Some(link) = node.attribute::<svgtree::Node>(AId::ClipPath) {
        clip_path = convert(link, state, cache);

        // Linked `clipPath` must be valid.
        if clip_path.is_none() {
            return None;
        }
    }

    let units = node.attribute(AId::ClipPathUnits).unwrap_or(Units::UserSpaceOnUse);
    let mut clip = ClipPath {
        id: node.element_id().to_string(),
        units,
        transform: node.attribute(AId::Transform).unwrap_or_default(),
        clip_path,
        root: Node::new(NodeKind::Group(Group::default())),
    };

    let mut clip_state = state.clone();
    clip_state.parent_clip_path = Some(node);
    converter::convert_clip_path_elements(node, &clip_state, cache, &mut clip.root);
    converter::ungroup_groups(clip.root.clone(), false);

    if clip.root.has_children() {
        let clip = Rc::new(clip);
        cache.clip_paths.insert(node.element_id().to_string(), clip.clone());
        Some(clip)
    } else {
        // A clip path without children is invalid.
        None
    }
}
