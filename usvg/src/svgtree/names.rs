// This file is autogenerated. Do not edit it!
// See ./codegen for details.

use std::fmt;

#[derive(Clone, Copy, PartialEq)]
pub enum EId {
    A,
    Circle,
    ClipPath,
    Defs,
    Ellipse,
    FeBlend,
    FeColorMatrix,
    FeComponentTransfer,
    FeComposite,
    FeConvolveMatrix,
    FeDiffuseLighting,
    FeDisplacementMap,
    FeDistantLight,
    FeFlood,
    FeFuncA,
    FeFuncB,
    FeFuncG,
    FeFuncR,
    FeGaussianBlur,
    FeImage,
    FeMerge,
    FeMergeNode,
    FeMorphology,
    FeOffset,
    FePointLight,
    FeSpecularLighting,
    FeSpotLight,
    FeTile,
    FeTurbulence,
    Filter,
    G,
    Image,
    Line,
    LinearGradient,
    Marker,
    Mask,
    Path,
    Pattern,
    Polygon,
    Polyline,
    RadialGradient,
    Rect,
    Stop,
    Style,
    Svg,
    Switch,
    Symbol,
    Text,
    TextPath,
    Tref,
    Tspan,
    Use
}

static ELEMENTS: Map<EId> = Map {
    key: 3558916427560184125,
    disps: &[
        (1, 13),
        (5, 3),
        (0, 19),
        (6, 24),
        (0, 0),
        (0, 3),
        (0, 1),
        (11, 10),
        (0, 21),
        (0, 0),
        (5, 0),
    ],
    entries: &[
        ("feConvolveMatrix", EId::FeConvolveMatrix),
        ("tspan", EId::Tspan),
        ("style", EId::Style),
        ("feFuncB", EId::FeFuncB),
        ("rect", EId::Rect),
        ("marker", EId::Marker),
        ("feDiffuseLighting", EId::FeDiffuseLighting),
        ("g", EId::G),
        ("symbol", EId::Symbol),
        ("pattern", EId::Pattern),
        ("path", EId::Path),
        ("feFuncR", EId::FeFuncR),
        ("a", EId::A),
        ("textPath", EId::TextPath),
        ("use", EId::Use),
        ("feFuncA", EId::FeFuncA),
        ("tref", EId::Tref),
        ("circle", EId::Circle),
        ("fePointLight", EId::FePointLight),
        ("defs", EId::Defs),
        ("feTile", EId::FeTile),
        ("image", EId::Image),
        ("stop", EId::Stop),
        ("feGaussianBlur", EId::FeGaussianBlur),
        ("feFlood", EId::FeFlood),
        ("polyline", EId::Polyline),
        ("feComponentTransfer", EId::FeComponentTransfer),
        ("linearGradient", EId::LinearGradient),
        ("feFuncG", EId::FeFuncG),
        ("ellipse", EId::Ellipse),
        ("clipPath", EId::ClipPath),
        ("feMerge", EId::FeMerge),
        ("feSpotLight", EId::FeSpotLight),
        ("feBlend", EId::FeBlend),
        ("svg", EId::Svg),
        ("feColorMatrix", EId::FeColorMatrix),
        ("feTurbulence", EId::FeTurbulence),
        ("feSpecularLighting", EId::FeSpecularLighting),
        ("switch", EId::Switch),
        ("feMorphology", EId::FeMorphology),
        ("feImage", EId::FeImage),
        ("feMergeNode", EId::FeMergeNode),
        ("feOffset", EId::FeOffset),
        ("polygon", EId::Polygon),
        ("feComposite", EId::FeComposite),
        ("radialGradient", EId::RadialGradient),
        ("line", EId::Line),
        ("feDisplacementMap", EId::FeDisplacementMap),
        ("feDistantLight", EId::FeDistantLight),
        ("mask", EId::Mask),
        ("text", EId::Text),
        ("filter", EId::Filter),
    ],
};

impl EId {
    pub fn from_str(text: &str) -> Option<EId> {
        ELEMENTS.get(text).cloned()
    }

    #[inline(never)]
    pub fn to_str(&self) -> &'static str {
        ELEMENTS.key(self)
    }
}

impl fmt::Debug for EId {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.to_str())
    }
}

impl fmt::Display for EId {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Clone, Copy, PartialEq)]
pub enum AId {
    Amplitude,
    BaselineShift,
    Class,
    ClipPath,
    ClipRule,
    ClipPathUnits,
    Color,
    ColorInterpolationFilters,
    Cx,
    Cy,
    D,
    Direction,
    Display,
    Dx,
    Dy,
    EnableBackground,
    Exponent,
    Fill,
    FillOpacity,
    FillRule,
    Filter,
    FilterUnits,
    FloodColor,
    FloodOpacity,
    FontFamily,
    FontSize,
    FontStretch,
    FontStyle,
    FontVariant,
    FontWeight,
    Fx,
    Fy,
    GradientTransform,
    GradientUnits,
    Height,
    Href,
    Id,
    ImageRendering,
    In,
    In2,
    Intercept,
    K1,
    K2,
    K3,
    K4,
    LetterSpacing,
    MarkerEnd,
    MarkerMid,
    MarkerStart,
    MarkerHeight,
    MarkerUnits,
    MarkerWidth,
    Mask,
    MaskContentUnits,
    MaskUnits,
    Mode,
    Offset,
    Opacity,
    Operator,
    Orient,
    Overflow,
    PatternContentUnits,
    PatternTransform,
    PatternUnits,
    Points,
    PreserveAspectRatio,
    PrimitiveUnits,
    R,
    RefX,
    RefY,
    RequiredExtensions,
    RequiredFeatures,
    Result,
    Rotate,
    Rx,
    Ry,
    ShapeRendering,
    Slope,
    Space,
    SpreadMethod,
    StartOffset,
    StdDeviation,
    StopColor,
    StopOpacity,
    Stroke,
    StrokeDasharray,
    StrokeDashoffset,
    StrokeLinecap,
    StrokeLinejoin,
    StrokeMiterlimit,
    StrokeOpacity,
    StrokeWidth,
    Style,
    SystemLanguage,
    TableValues,
    TextAnchor,
    TextDecoration,
    TextRendering,
    Transform,
    Type,
    Values,
    ViewBox,
    Visibility,
    Width,
    WordSpacing,
    WritingMode,
    X,
    X1,
    X2,
    Y,
    Y1,
    Y2
}

static ATTRIBUTES: Map<AId> = Map {
    key: 732231254413039614,
    disps: &[
        (0, 96),
        (4, 30),
        (4, 8),
        (1, 43),
        (1, 108),
        (1, 0),
        (1, 79),
        (6, 61),
        (1, 31),
        (49, 0),
        (0, 3),
        (1, 3),
        (0, 7),
        (0, 0),
        (0, 64),
        (1, 41),
        (1, 34),
        (0, 51),
        (2, 82),
        (0, 90),
        (0, 6),
        (0, 72),
        (42, 36),
    ],
    entries: &[
        ("stroke-dashoffset", AId::StrokeDashoffset),
        ("requiredFeatures", AId::RequiredFeatures),
        ("points", AId::Points),
        ("visibility", AId::Visibility),
        ("font-style", AId::FontStyle),
        ("k3", AId::K3),
        ("marker-start", AId::MarkerStart),
        ("overflow", AId::Overflow),
        ("k4", AId::K4),
        ("y", AId::Y),
        ("rotate", AId::Rotate),
        ("maskContentUnits", AId::MaskContentUnits),
        ("systemLanguage", AId::SystemLanguage),
        ("display", AId::Display),
        ("color-interpolation-filters", AId::ColorInterpolationFilters),
        ("width", AId::Width),
        ("stop-opacity", AId::StopOpacity),
        ("font-family", AId::FontFamily),
        ("font-weight", AId::FontWeight),
        ("filter", AId::Filter),
        ("href", AId::Href),
        ("cy", AId::Cy),
        ("stroke", AId::Stroke),
        ("fy", AId::Fy),
        ("stroke-dasharray", AId::StrokeDasharray),
        ("requiredExtensions", AId::RequiredExtensions),
        ("height", AId::Height),
        ("result", AId::Result),
        ("orient", AId::Orient),
        ("mask", AId::Mask),
        ("operator", AId::Operator),
        ("in2", AId::In2),
        ("enable-background", AId::EnableBackground),
        ("id", AId::Id),
        ("stroke-linejoin", AId::StrokeLinejoin),
        ("text-decoration", AId::TextDecoration),
        ("x2", AId::X2),
        ("viewBox", AId::ViewBox),
        ("values", AId::Values),
        ("fill", AId::Fill),
        ("letter-spacing", AId::LetterSpacing),
        ("ry", AId::Ry),
        ("intercept", AId::Intercept),
        ("transform", AId::Transform),
        ("stop-color", AId::StopColor),
        ("in", AId::In),
        ("refX", AId::RefX),
        ("flood-opacity", AId::FloodOpacity),
        ("marker-end", AId::MarkerEnd),
        ("clip-path", AId::ClipPath),
        ("marker-mid", AId::MarkerMid),
        ("fill-opacity", AId::FillOpacity),
        ("spreadMethod", AId::SpreadMethod),
        ("word-spacing", AId::WordSpacing),
        ("primitiveUnits", AId::PrimitiveUnits),
        ("cx", AId::Cx),
        ("x1", AId::X1),
        ("markerWidth", AId::MarkerWidth),
        ("preserveAspectRatio", AId::PreserveAspectRatio),
        ("r", AId::R),
        ("writing-mode", AId::WritingMode),
        ("direction", AId::Direction),
        ("class", AId::Class),
        ("maskUnits", AId::MaskUnits),
        ("flood-color", AId::FloodColor),
        ("y1", AId::Y1),
        ("dy", AId::Dy),
        ("baseline-shift", AId::BaselineShift),
        ("patternContentUnits", AId::PatternContentUnits),
        ("filterUnits", AId::FilterUnits),
        ("markerHeight", AId::MarkerHeight),
        ("startOffset", AId::StartOffset),
        ("amplitude", AId::Amplitude),
        ("slope", AId::Slope),
        ("stroke-miterlimit", AId::StrokeMiterlimit),
        ("offset", AId::Offset),
        ("type", AId::Type),
        ("style", AId::Style),
        ("shape-rendering", AId::ShapeRendering),
        ("font-size", AId::FontSize),
        ("clipPathUnits", AId::ClipPathUnits),
        ("y2", AId::Y2),
        ("rx", AId::Rx),
        ("tableValues", AId::TableValues),
        ("x", AId::X),
        ("space", AId::Space),
        ("opacity", AId::Opacity),
        ("dx", AId::Dx),
        ("stroke-opacity", AId::StrokeOpacity),
        ("clip-rule", AId::ClipRule),
        ("stroke-linecap", AId::StrokeLinecap),
        ("font-stretch", AId::FontStretch),
        ("patternUnits", AId::PatternUnits),
        ("fx", AId::Fx),
        ("d", AId::D),
        ("gradientTransform", AId::GradientTransform),
        ("k1", AId::K1),
        ("text-anchor", AId::TextAnchor),
        ("stdDeviation", AId::StdDeviation),
        ("refY", AId::RefY),
        ("stroke-width", AId::StrokeWidth),
        ("patternTransform", AId::PatternTransform),
        ("mode", AId::Mode),
        ("image-rendering", AId::ImageRendering),
        ("k2", AId::K2),
        ("color", AId::Color),
        ("fill-rule", AId::FillRule),
        ("font-variant", AId::FontVariant),
        ("text-rendering", AId::TextRendering),
        ("markerUnits", AId::MarkerUnits),
        ("gradientUnits", AId::GradientUnits),
        ("exponent", AId::Exponent),
    ],
};

impl AId {
    pub fn from_str(text: &str) -> Option<AId> {
        ATTRIBUTES.get(text).cloned()
    }

    #[inline(never)]
    pub fn to_str(&self) -> &'static str {
        ATTRIBUTES.key(self)
    }
}

impl fmt::Debug for AId {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.to_str())
    }
}

impl fmt::Display for AId {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

// A stripped down `phf` crate fork.
//
// https://github.com/sfackler/rust-phf

struct Map<V: 'static> {
    pub key: u64,
    pub disps: &'static [(u32, u32)],
    pub entries: &'static[(&'static str, V)],
}

impl<V: PartialEq> Map<V> {
    fn get(&self, key: &str) -> Option<&V> {
        use std::borrow::Borrow;

        let hash = hash(key, self.key);
        let index = get_index(hash, &*self.disps, self.entries.len());
        let entry = &self.entries[index as usize];
        let b = entry.0.borrow();
        if b == key {
            Some(&entry.1)
        } else {
            None
        }
    }

    fn key(&self, value: &V) -> &'static str {
        self.entries.iter().find(|kv| kv.1 == *value).unwrap().0
    }
}

#[inline]
fn hash(x: &str, key: u64) -> u64 {
    use std::hash::Hasher;

    let mut hasher = siphasher::sip::SipHasher13::new_with_keys(0, key);
    hasher.write(x.as_bytes());
    hasher.finish()
}

#[inline]
fn get_index(hash: u64, disps: &[(u32, u32)], len: usize) -> u32 {
    let (g, f1, f2) = split(hash);
    let (d1, d2) = disps[(g % (disps.len() as u32)) as usize];
    displace(f1, f2, d1, d2) % (len as u32)
}

#[inline]
fn split(hash: u64) -> (u32, u32, u32) {
    const BITS: u32 = 21;
    const MASK: u64 = (1 << BITS) - 1;

    ((hash & MASK) as u32,
     ((hash >> BITS) & MASK) as u32,
     ((hash >> (2 * BITS)) & MASK) as u32)
}

#[inline]
fn displace(f1: u32, f2: u32, d1: u32, d2: u32) -> u32 {
    d2 + f1 * d1 + f2
}
