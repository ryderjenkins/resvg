// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use std::fmt;
use std::collections::HashMap;

use log::warn;

use svgtypes::FuzzyEq;

use crate::geom::Rect;
use crate::tree;

mod parse;
pub use parse::*;

mod names;
pub use names::*;

type Range = std::ops::Range<usize>;


pub struct Document {
    nodes: Vec<NodeData>,
    attrs: Vec<Attribute>,
    links: HashMap<String, NodeId>,
}

impl Document {
    #[inline]
    pub fn root(&self) -> Node {
        Node { id: NodeId(0), d: &self.nodes[0], doc: self }
    }

    pub fn root_element(&self) -> Node {
        // `unwrap` is safe, because `Document` is guarantee to have at least one element.
        self.root().first_element_child().unwrap()
    }

    pub fn descendants(&self) -> Descendants {
        self.root().descendants()
    }

    #[inline]
    pub fn element_by_id(&self, id: &str) -> Option<Node> {
        let node_id = self.links.get(id)?;
        Some(self.get(*node_id))
    }

    #[inline]
    pub fn get(&self, id: NodeId) -> Node {
        Node { id, d: &self.nodes[id.0], doc: self }
    }
}

impl fmt::Debug for Document {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        if !self.root().has_children() {
            return write!(f, "Document []");
        }

        macro_rules! writeln_indented {
            ($depth:expr, $f:expr, $fmt:expr) => {
                for _ in 0..$depth { write!($f, "    ")?; }
                writeln!($f, $fmt)?;
            };
            ($depth:expr, $f:expr, $fmt:expr, $($arg:tt)*) => {
                for _ in 0..$depth { write!($f, "    ")?; }
                writeln!($f, $fmt, $($arg)*)?;
            };
        }

        fn print_children(parent: Node, depth: usize, f: &mut fmt::Formatter)
            -> Result<(), fmt::Error>
        {
            for child in parent.children() {
                if child.is_element() {
                    writeln_indented!(depth, f, "Element {{");
                    writeln_indented!(depth, f, "    tag_name: {:?}", child.tag_name());

                    if !child.attributes().is_empty() {
                        writeln_indented!(depth + 1, f, "attributes: [");
                        for attr in child.attributes() {
                            writeln_indented!(depth + 2, f, "{:?}", attr);
                        }
                        writeln_indented!(depth + 1, f, "]");
                    }

                    if child.has_children() {
                        writeln_indented!(depth, f, "    children: [");
                        print_children(child, depth + 2, f)?;
                        writeln_indented!(depth, f, "    ]");
                    }

                    writeln_indented!(depth, f, "}}");
                } else {
                    writeln_indented!(depth, f, "{:?}", child);
                }
            }

            Ok(())
        }

        writeln!(f, "Document [")?;
        print_children(self.root(), 1, f)?;
        writeln!(f, "]")?;

        Ok(())
    }
}


#[derive(Clone, Copy, PartialEq, Debug)]
pub enum NodeType {
    Root,
    Element,
    Text,
}


#[derive(Clone, Copy, PartialEq, Debug)]
pub struct NodeId(usize);

#[derive(Clone, Copy, PartialEq, Debug)]
struct AttributeId(usize);


enum NodeKind {
    Root,
    Element {
        tag_name: EId,
        attributes: Range,
    },
    Text(String),
}


struct NodeData {
    parent: Option<NodeId>,
    prev_sibling: Option<NodeId>,
    next_sibling: Option<NodeId>,
    children: Option<(NodeId, NodeId)>,
    kind: NodeKind,
}

#[derive(Clone, Debug)]
pub enum AttributeValue {
    None,
    CurrentColor,
    Angle(svgtypes::Angle),
    AspectRatio(svgtypes::AspectRatio),
    Color(svgtypes::Color),
    Length(svgtypes::Length),
    Link(String),
    Number(f64),
    NumberList(svgtypes::NumberList),
    Opacity(tree::Opacity),
    Paint(String, Option<svgtypes::PaintFallback>),
    Path(tree::PathData),
    String(String),
    Transform(svgtypes::Transform),
    ViewBox(svgtypes::ViewBox),
}

#[derive(Clone)]
pub struct Attribute {
    pub name: AId,
    pub value: AttributeValue,
}

impl fmt::Debug for Attribute {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        write!(f, "Attribute {{ name: {:?}, value: {:?} }}", self.name, self.value)
    }
}


#[derive(Clone, Copy)]
pub struct Node<'a> {
    id: NodeId,
    doc: &'a Document,
    d: &'a NodeData,
}

impl Eq for Node<'_> {}

impl PartialEq for Node<'_> {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
           self.id == other.id
        && self.doc as *const _ == other.doc as *const _
        && self.d as *const _ == other.d as *const _
    }
}

impl<'a> Node<'a> {
    #[inline]
    pub fn id(&self) -> NodeId {
        self.id
    }

    #[inline]
    pub fn node_type(&self) -> NodeType {
        match self.d.kind {
            NodeKind::Root => NodeType::Root,
            NodeKind::Element { .. } => NodeType::Element,
            NodeKind::Text(_) => NodeType::Text,
        }
    }

    #[allow(dead_code)]
    #[inline]
    pub fn is_root(&self) -> bool {
        self.node_type() == NodeType::Root
    }

    #[inline]
    pub fn is_element(&self) -> bool {
        self.node_type() == NodeType::Element
    }

    #[inline]
    pub fn is_text(&self) -> bool {
        self.node_type() == NodeType::Text
    }

    #[inline]
    pub fn document(&self) -> &'a Document {
        self.doc
    }

    #[inline]
    pub fn tag_name(&self) -> Option<EId> {
        match self.d.kind {
            NodeKind::Element { tag_name, .. } => Some(tag_name),
            _ => None,
        }
    }

    #[inline]
    pub fn has_tag_name(&self, name: EId) -> bool {
        match self.d.kind {
            NodeKind::Element { tag_name, .. } => tag_name == name,
            _ => false,
        }
    }

    #[inline]
    pub fn element_id(&self) -> &str {
        self.attribute(AId::Id).unwrap_or("")
    }

    #[inline]
    pub fn attribute<V: FromValue<'a>>(&self, aid: AId) -> Option<V> {
        FromValue::get(*self, aid)
    }

    #[inline]
    pub fn has_attribute(&self, aid: AId) -> bool {
        self.attributes().iter().any(|a| a.name == aid)
    }

    #[inline]
    pub fn attributes(&self) -> &'a [Attribute] {
        match self.d.kind {
            NodeKind::Element { ref attributes, .. } => &self.doc.attrs[attributes.clone()],
            _ => &[],
        }
    }

    #[inline]
    fn attribute_id(&self, aid: AId) -> Option<AttributeId> {
        match self.d.kind {
            NodeKind::Element { ref attributes, .. } => {
                let idx = self.attributes().iter().position(|attr| attr.name == aid)?;
                Some(AttributeId(attributes.start + idx))
            }
            _ => None,
        }
    }

    pub fn find_attribute<V: FromValue<'a>>(&self, aid: AId) -> Option<V> {
        if aid.is_inheritable() {
            for n in self.ancestors() {
                if n.has_attribute(aid) {
                    return n.attribute(aid);
                }
            }
        } else {
            return match self.attribute(aid) {
                Some(v) => Some(v),
                None => self.parent_element()?.attribute(aid),
            };
        }

        None
    }

    pub fn find_node_with_attribute(&self, aid: AId) -> Option<Node> {
        for n in self.ancestors() {
            if n.has_attribute(aid) {
                return Some(n);
            }
        }

        None
    }

    pub fn has_valid_transform(&self, aid: AId) -> bool {
        // Do not use Node::attribute::<Transform>, because it will always
        // return a valid transform.

        let attr = match self.attributes().iter().find(|a| a.name == aid) {
            Some(attr) => attr,
            None => return true,
        };

        if let AttributeValue::Transform(ref ts) = attr.value {
            let (sx, sy) = ts.get_scale();
            if sx.fuzzy_eq(&0.0) || sy.fuzzy_eq(&0.0) {
                return false;
            }
        }

        true
    }

    pub fn get_viewbox(&self) -> Option<Rect> {
        let vb: svgtypes::ViewBox = self.attribute(AId::ViewBox)?;
        Rect::new(vb.x, vb.y, vb.w, vb.h)
    }

    pub fn text(&self) -> &'a str {
        match self.d.kind {
            NodeKind::Element { .. } => {
                match self.first_child() {
                    Some(child) if child.is_text() => {
                        match self.doc.nodes[child.id.0].kind {
                            NodeKind::Text(ref text) => text,
                            _ => ""
                        }
                    }
                    _ => "",
                }
            }
            NodeKind::Text(ref text) => text,
            _ => "",
        }
    }

    #[inline]
    fn gen_node(&self, id: NodeId) -> Node<'a> {
        Node { id, d: &self.doc.nodes[id.0], doc: self.doc }
    }

    #[inline]
    pub fn parent(&self) -> Option<Self> {
        self.d.parent.map(|id| self.gen_node(id))
    }

    pub fn parent_element(&self) -> Option<Self> {
        self.ancestors().skip(1).filter(|n| n.is_element()).nth(0)
    }

    #[inline]
    pub fn prev_sibling(&self) -> Option<Self> {
        self.d.prev_sibling.map(|id| self.gen_node(id))
    }

    #[inline]
    pub fn next_sibling(&self) -> Option<Self> {
        self.d.next_sibling.map(|id| self.gen_node(id))
    }

    #[inline]
    pub fn first_child(&self) -> Option<Self> {
        self.d.children.map(|(id, _)| self.gen_node(id))
    }

    pub fn first_element_child(&self) -> Option<Self> {
        self.children().filter(|n| n.is_element()).nth(0)
    }

    #[inline]
    pub fn last_child(&self) -> Option<Self> {
        self.d.children.map(|(_, id)| self.gen_node(id))
    }

    #[inline]
    pub fn has_children(&self) -> bool {
        self.d.children.is_some()
    }

    /// Returns an iterator over ancestor nodes starting at this node.
    #[inline]
    pub fn ancestors(&self) -> Ancestors<'a> {
        Ancestors(Some(*self))
    }

    /// Returns an iterator over children nodes.
    #[inline]
    pub fn children(&self) -> Children<'a> {
        Children { front: self.first_child(), back: self.last_child() }
    }

    /// Returns an iterator which traverses the subtree starting at this node.
    #[inline]
    pub fn traverse(&self) -> Traverse<'a> {
        Traverse { root: *self, edge: None }
    }

    /// Returns an iterator over this node and its descendants.
    #[inline]
    pub fn descendants(&self) -> Descendants<'a> {
        Descendants(self.traverse())
    }

    pub fn href_iter(&self) -> HrefIter {
        HrefIter {
            doc: self.document(),
            origin: self.id(),
            curr: self.id(),
            is_first: true,
            is_finished: false,
        }
    }
}

impl fmt::Debug for Node<'_> {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        match self.d.kind {
            NodeKind::Root => write!(f, "Root"),
            NodeKind::Element { .. } => {
                write!(f, "Element {{ tag_name: {:?}, attributes: {:?} }}",
                       self.tag_name(), self.attributes())
            }
            NodeKind::Text(ref text) => write!(f, "Text({:?})", text),
        }
    }
}

macro_rules! axis_iterators {
    ($($i:ident($f:path);)*) => {
        $(
            #[derive(Clone)]
            pub struct $i<'a>(Option<Node<'a>>);

            impl<'a> Iterator for $i<'a> {
                type Item = Node<'a>;

                #[inline]
                fn next(&mut self) -> Option<Self::Item> {
                    let node = self.0.take();
                    self.0 = node.as_ref().and_then($f);
                    node
                }
            }
        )*
    };
}

axis_iterators! {
    Ancestors(Node::parent);
    PrevSiblings(Node::prev_sibling);
    NextSiblings(Node::next_sibling);
}


#[derive(Clone)]
pub struct Children<'a> {
    front: Option<Node<'a>>,
    back: Option<Node<'a>>,
}

impl<'a> Iterator for Children<'a> {
    type Item = Node<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.front == self.back {
            let node = self.front.take();
            self.back = None;
            node
        } else {
            let node = self.front.take();
            self.front = node.as_ref().and_then(Node::next_sibling);
            node
        }
    }
}

impl<'a> DoubleEndedIterator for Children<'a> {
    fn next_back(&mut self) -> Option<Self::Item> {
        if self.back == self.front {
            let node = self.back.take();
            self.front = None;
            node
        } else {
            let node = self.back.take();
            self.back = node.as_ref().and_then(Node::prev_sibling);
            node
        }
    }
}


#[derive(Clone, Copy, PartialEq, Debug)]
pub enum Edge<'a> {
    Open(Node<'a>),
    Close(Node<'a>),
}


#[derive(Clone)]
pub struct Traverse<'a> {
    root: Node<'a>,
    edge: Option<Edge<'a>>,
}

impl<'a> Iterator for Traverse<'a> {
    type Item = Edge<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        match self.edge {
            Some(Edge::Open(node)) => {
                self.edge = Some(match node.first_child() {
                    Some(first_child) => Edge::Open(first_child),
                    None => Edge::Close(node),
                });
            }
            Some(Edge::Close(node)) => {
                if node == self.root {
                    self.edge = None;
                } else if let Some(next_sibling) = node.next_sibling() {
                    self.edge = Some(Edge::Open(next_sibling));
                } else {
                    self.edge = node.parent().map(Edge::Close);
                }
            }
            None => {
                self.edge = Some(Edge::Open(self.root));
            }
        }

        self.edge
    }
}


#[derive(Clone)]
pub struct Descendants<'a>(Traverse<'a>);

impl<'a> Iterator for Descendants<'a> {
    type Item = Node<'a>;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        for edge in &mut self.0 {
            if let Edge::Open(node) = edge {
                return Some(node);
            }
        }

        None
    }
}


pub struct HrefIter<'a> {
    doc: &'a Document,
    origin: NodeId,
    curr: NodeId,
    is_first: bool,
    is_finished: bool,
}

impl<'a> Iterator for HrefIter<'a> {
    type Item = NodeId;

    fn next(&mut self) -> Option<Self::Item> {
        if self.is_finished {
            return None;
        }

        if self.is_first {
            self.is_first = false;
            return Some(self.curr);
        }

        if let Some(link) = self.doc.get(self.curr).attribute::<Node>(AId::Href) {
            if link.id() == self.curr || link.id() == self.origin {
                warn!(
                    "Element '#{}' cannot reference itself via 'xlink:href'.",
                    self.doc.get(self.origin).element_id()
                );
                self.is_finished = true;
                return None;
            }

            self.curr = link.id();
            Some(link.id())
        } else {
            None
        }
    }
}


pub trait FromValue<'a>: Sized {
    fn get(node: Node<'a>, aid: AId) -> Option<Self>;
}

macro_rules! impl_from_value {
    ($rtype:ty, $etype:ident) => (
        impl FromValue<'_> for $rtype {
            fn get(node: Node, aid: AId) -> Option<Self> {
                let a = node.attributes().iter().find(|a| a.name == aid)?;
                if let AttributeValue::$etype(ref v) = a.value { Some(*v) } else { None }
            }
        }
    )
}

impl_from_value!(svgtypes::Color, Color);
impl_from_value!(svgtypes::Length, Length);
impl_from_value!(svgtypes::ViewBox, ViewBox);
impl_from_value!(svgtypes::AspectRatio, AspectRatio);
impl_from_value!(svgtypes::Angle, Angle);
impl_from_value!(f64, Number);
impl_from_value!(crate::tree::Opacity, Opacity);

impl<'a> FromValue<'a> for &'a AttributeValue {
    fn get(node: Node<'a>, aid: AId) -> Option<Self> {
        node.attributes().iter().find(|a| a.name == aid).map(|a| &a.value)
    }
}

impl<'a> FromValue<'a> for svgtypes::Transform {
    fn get(node: Node<'a>, aid: AId) -> Option<Self> {
        let a = node.attributes().iter().find(|a| a.name == aid)?;
        let ts = match a.value {
            AttributeValue::Transform(ref ts) => ts,
            _ => return None,
        };

        let (sx, sy) = ts.get_scale();
        if sx.fuzzy_eq(&0.0) || sy.fuzzy_eq(&0.0) {
            Some(svgtypes::Transform::default())
        } else {
            Some(*ts)
        }
    }
}

impl FromValue<'_> for tree::PathData {
    fn get(node: Node, aid: AId) -> Option<Self> {
        let a = node.attributes().iter().find(|a| a.name == aid)?;
        if let AttributeValue::Path(ref v) = a.value { Some(v.clone()) } else { None }
    }
}

impl<'a> FromValue<'a> for &'a svgtypes::NumberList {
    fn get(node: Node<'a>, aid: AId) -> Option<Self> {
        let a = node.attributes().iter().find(|a| a.name == aid)?;
        if let AttributeValue::NumberList(ref v) = a.value { Some(v) } else { None }
    }
}

impl<'a> FromValue<'a> for &'a str {
    fn get(node: Node<'a>, aid: AId) -> Option<Self> {
        let a = node.attributes().iter().find(|a| a.name == aid)?;
        match a.value {
            AttributeValue::None => {
                // A special case, because matching `None` is too verbose.
                //
                // match node.attribute(AId::Display) {
                //     Some(&svgtree::AttributeValue::None) => true,
                //     None => false,
                // }
                //
                // vs
                //
                // node.attribute(AId::Display) == Some("none")
                Some("none")
            }
            AttributeValue::String(ref v) => Some(v.as_str()),
            _ => None,
        }
    }
}

impl<'a> FromValue<'a> for Node<'a> {
    fn get(node: Node<'a>, aid: AId) -> Option<Self> {
        let a = node.attributes().iter().find(|a| a.name == aid)?;
        let id = match a.value  {
            AttributeValue::Link(ref id) => id,
            _ => return None,
        };

        node.document().element_by_id(&id)
    }
}

pub trait EnumFromStr: Sized {
    fn enum_from_str(text: &str) -> Option<Self>;
}

impl<'a, T: EnumFromStr> FromValue<'a> for T {
    #[inline]
    fn get(node: Node, aid: AId) -> Option<Self> {
        EnumFromStr::enum_from_str(node.attribute(aid)?)
    }
}


impl EId {
    #[inline]
    pub fn is_graphic(&self) -> bool {
        matches!(self,
              EId::Circle
            | EId::Ellipse
            | EId::Image
            | EId::Line
            | EId::Path
            | EId::Polygon
            | EId::Polyline
            | EId::Rect
            | EId::Text
            | EId::Use
        )
    }

    #[inline]
    pub fn is_gradient(&self) -> bool {
        matches!(self,
              EId::LinearGradient
            | EId::RadialGradient
        )
    }

    #[inline]
    pub fn is_paint_server(&self) -> bool {
        matches!(self,
              EId::LinearGradient
            | EId::RadialGradient
            | EId::Pattern
        )
    }
}

impl AId {
    #[inline]
    pub fn is_presentation(&self) -> bool {
        matches!(self,
              AId::BaselineShift
            | AId::ClipPath
            | AId::ClipRule
            | AId::Color
            | AId::ColorInterpolationFilters
            | AId::Direction
            | AId::Display
            | AId::Fill
            | AId::FillOpacity
            | AId::FillRule
            | AId::Filter
            | AId::FloodColor
            | AId::FloodOpacity
            | AId::FontFamily
            | AId::FontSize
            | AId::FontStretch
            | AId::FontStyle
            | AId::FontVariant
            | AId::FontWeight
            | AId::ImageRendering
            | AId::LetterSpacing
            | AId::MarkerEnd
            | AId::MarkerMid
            | AId::MarkerStart
            | AId::Mask
            | AId::Opacity
            | AId::Overflow
            | AId::ShapeRendering
            | AId::StopColor
            | AId::StopOpacity
            | AId::Stroke
            | AId::StrokeDasharray
            | AId::StrokeDashoffset
            | AId::StrokeLinecap
            | AId::StrokeLinejoin
            | AId::StrokeMiterlimit
            | AId::StrokeOpacity
            | AId::StrokeWidth
            | AId::TextAnchor
            | AId::TextDecoration
            | AId::TextRendering
            | AId::Visibility
            | AId::WordSpacing
            | AId::WritingMode)
    }

    #[inline]
    pub fn is_inheritable(&self) -> bool {
        if self.is_presentation() {
            !is_non_inheritable(*self)
        } else {
            false
        }
    }

    #[inline]
    pub fn allows_inherit_value(&self) -> bool {
        matches!(self,
              AId::BaselineShift
            | AId::ClipPath
            | AId::ClipRule
            | AId::Color
            | AId::ColorInterpolationFilters
            | AId::Direction
            | AId::Display
            | AId::Fill
            | AId::FillOpacity
            | AId::FillRule
            | AId::Filter
            | AId::FloodColor
            | AId::FloodOpacity
            | AId::FontFamily
            | AId::FontSize
            | AId::FontStretch
            | AId::FontStyle
            | AId::FontVariant
            | AId::FontWeight
            | AId::ImageRendering
            | AId::LetterSpacing
            | AId::MarkerEnd
            | AId::MarkerMid
            | AId::MarkerStart
            | AId::Mask
            | AId::Opacity
            | AId::Overflow
            | AId::ShapeRendering
            | AId::StopColor
            | AId::StopOpacity
            | AId::Stroke
            | AId::StrokeDasharray
            | AId::StrokeDashoffset
            | AId::StrokeLinecap
            | AId::StrokeLinejoin
            | AId::StrokeMiterlimit
            | AId::StrokeOpacity
            | AId::StrokeWidth
            | AId::TextAnchor
            | AId::TextDecoration
            | AId::TextRendering
            | AId::Visibility
            | AId::WordSpacing
            | AId::WritingMode)
    }
}

#[inline]
fn is_non_inheritable(id: AId) -> bool {
    matches!(id,
          AId::BaselineShift
        | AId::ClipPath
        | AId::Display
        | AId::Filter
        | AId::FloodColor
        | AId::FloodOpacity
        | AId::Mask
        | AId::Opacity
        | AId::Overflow
        | AId::StopColor
        | AId::StopOpacity
        | AId::TextDecoration
        | AId::Visibility)
}
