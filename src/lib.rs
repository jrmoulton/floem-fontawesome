use floem::style::TextColor;
use floem::views::{SvgCssPropExtractor, SvgOrStyle, brush_to_css_string};
use floem::{
    View, ViewId,
    peniko::Brush,
    prop, prop_extractor,
    style::{CustomStyle, Style, StylePropValue, Transition},
    style_class,
    views::Decorators,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct FaVariant {
    // Base style
    pub base: FaBaseStyle,
    // Modifiers
    pub is_sharp: bool,
    pub is_duotone: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FaBaseStyle {
    Solid,
    Regular,
    Light,
    Thin,
    Brands,
}

impl Default for FaVariant {
    fn default() -> Self {
        Self {
            base: FaBaseStyle::Solid,
            is_sharp: false,
            is_duotone: false,
        }
    }
}
impl std::fmt::Display for FaVariant {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut parts = Vec::new();
        if self.is_sharp {
            parts.push("sharp");
        }
        if self.is_duotone {
            parts.push("duotone");
        }
        parts.push(match self.base {
            FaBaseStyle::Solid => "solid",
            FaBaseStyle::Regular => "regular",
            FaBaseStyle::Light => "light",
            FaBaseStyle::Thin => "thin",
            FaBaseStyle::Brands => "brands",
        });
        write!(f, "{}", parts.join("-"))
    }
}

impl FaVariant {
    pub fn is_duotone(&self) -> bool {
        self.is_duotone
    }
}

impl StylePropValue for FaVariant {}

prop!(pub FaVariantProp: FaVariant {} = FaVariant::default());

// Property for color (for non-duotone variants)
prop!(pub FaColor: Option<Brush> {} = None);

// Duotone specific colors properties
prop!(pub FaPrimary: Option<Brush> {} = None);
prop!(pub FaSecondary: Option<Brush> {} = None);

pub trait FaIconTrait {
    fn svg(&self, variant: FaVariant) -> &'static str;
}

pub struct FaIcon<T> {
    id: ViewId,
    svg_id: ViewId,
    style: FaStyleExt,
    icon: T,
    variant: FaVariant,
}
impl<T: FaIconTrait> View for FaIcon<T> {
    fn id(&self) -> ViewId {
        self.id
    }
    fn style_pass(&mut self, cx: &mut floem::context::StyleCx<'_>) {
        if self.style.read(cx) {
            self.variant = self.style.variant();
            self.svg_id
                .update_state(SvgOrStyle::Svg(self.icon.svg(self.variant).to_string()));
            self.svg_id
                .update_state(SvgOrStyle::Style(self.style.css_string()));
        }
        cx.style_view(self.svg_id);
    }
}
pub fn fa_icon<T: FaIconTrait + 'static>(icon: T) -> FaIcon<T> {
    let id = ViewId::new();
    let variant = FaVariant::default();
    let svg = icon.svg(variant);
    let svg = floem::views::svg(svg).style(|s| s.size_full());
    let svg_id = svg.id();
    id.set_children([svg]);
    FaIcon {
        id,
        svg_id,
        style: Default::default(),
        icon,
        variant,
    }
    .class(FaClass)
}

style_class!(pub FaClass);

prop_extractor! {
    pub FaStyleExt {
        variant: FaVariantProp,
        color: FaColor,
        primary: FaPrimary,
        secondary: FaSecondary,
    }
}

impl SvgCssPropExtractor for FaStyleExt {
    fn read_custom(&mut self, cx: &mut floem::context::StyleCx) -> bool {
        self.read(cx)
    }

    fn css_string(&self) -> String {
        let variant = self.variant();

        if variant.is_duotone() {
            // Duotone styling
            let primary_color = match self.primary() {
                Some(ref brush) => brush_to_css_string(brush),
                None => "currentColor".to_string(),
            };
            let secondary_color = match self.secondary() {
                Some(ref brush) => brush_to_css_string(brush),
                None => "currentColor".to_string(),
            };

            format!(
                r#"
                    .fa-primary {{
                      fill: {} !important;
                    }}
                    .fa-secondary {{
                      fill: {} !important;
                    }}
                "#,
                primary_color, secondary_color
            )
        } else {
            // For non-duotone variants, use single color
            match self.color() {
                Some(ref brush) => {
                    let color = brush_to_css_string(brush);
                    format!(
                        r#"
                            svg {{
                              fill: {} !important;
                            }}
                        "#,
                        color
                    )
                }
                None => "".to_string(),
            }
        }
    }
}

#[derive(Debug, Clone, Default)]
pub struct FaStyle(Style);

impl From<FaStyle> for Style {
    fn from(value: FaStyle) -> Self {
        value.0.set(TextColor, None)
    }
}

impl From<Style> for FaStyle {
    fn from(value: Style) -> Self {
        let value = value.set(TextColor, None);
        Self(value)
    }
}

impl CustomStyle for FaStyle {
    type StyleClass = FaClass;
}

impl FaStyle {
    // Base styles
    pub fn solid(mut self) -> Self {
        let mut var = self.0.get(FaVariantProp);
        var.base = FaBaseStyle::Solid;
        self = Self(self.0.set(FaVariantProp, var));
        self
    }

    pub fn regular(mut self) -> Self {
        let mut var = self.0.get(FaVariantProp);
        var.base = FaBaseStyle::Regular;
        self = Self(self.0.set(FaVariantProp, var));
        self
    }

    pub fn light(mut self) -> Self {
        let mut var = self.0.get(FaVariantProp);
        var.base = FaBaseStyle::Light;
        self = Self(self.0.set(FaVariantProp, var));
        self
    }

    pub fn thin(mut self) -> Self {
        let mut var = self.0.get(FaVariantProp);
        var.base = FaBaseStyle::Thin;
        self = Self(self.0.set(FaVariantProp, var));
        self
    }

    pub fn brands(mut self) -> Self {
        let mut var = self.0.get(FaVariantProp);
        var.base = FaBaseStyle::Brands;
        self = Self(self.0.set(FaVariantProp, var));
        self
    }

    // Modifiers
    pub fn sharp(mut self) -> Self {
        let mut var = self.0.get(FaVariantProp);
        var.is_sharp = true;
        self = Self(self.0.set(FaVariantProp, var));
        self
    }

    pub fn duotone(mut self) -> Self {
        let mut var = self.0.get(FaVariantProp);
        var.is_duotone = true;
        self = Self(self.0.set(FaVariantProp, var));
        self
    }

    // Color settings
    pub fn color(mut self, color: impl Into<Brush>) -> Self {
        self = Self(self.0.set(FaColor, Some(color.into())));
        self
    }

    pub fn primary(mut self, color: impl Into<Brush>) -> Self {
        self = Self(self.0.set(FaPrimary, Some(color.into())));
        self
    }

    pub fn secondary(mut self, color: impl Into<Brush>) -> Self {
        self = Self(self.0.set(FaSecondary, Some(color.into())));
        self
    }

    // Transitions
    pub fn transition_color(mut self, transition: Transition) -> Self {
        self = Self(self.0.transition(FaColor, transition));
        self
    }

    pub fn transition_primary(mut self, transition: Transition) -> Self {
        self = Self(self.0.transition(FaPrimary, transition));
        self
    }

    pub fn transition_secondary(mut self, transition: Transition) -> Self {
        self = Self(self.0.transition(FaSecondary, transition));
        self
    }
}

#[macro_export]
macro_rules! fa_icon_build {
    ($base_path:expr, $public:vis enum {
        $($variants:tt)*
    }) => {

        extract_enum_variants!{[$($variants)*], $public}

        impl floem::IntoView for FaIcon {
            type V = $crate::font_awesome::FaIcon<FaIcon>;

            fn into_view(self) -> Self::V {
                $crate::font_awesome::fa_icon(self)
            }
        }

        impl $crate::font_awesome::FaIconTrait for FaIcon {
            fn svg(&self, variant: $crate::font_awesome::FaVariant) -> &'static str {
                use $crate::font_awesome::{FaVariant, FaBaseStyle};

                // Get the variant string for file path
                let variant_path = match variant {
                    FaVariant { base: FaBaseStyle::Solid, is_sharp: false, is_duotone: false } => "solid",
                    FaVariant { base: FaBaseStyle::Regular, is_sharp: false, is_duotone: false } => "regular",
                    FaVariant { base: FaBaseStyle::Light, is_sharp: false, is_duotone: false } => "light",
                    FaVariant { base: FaBaseStyle::Thin, is_sharp: false, is_duotone: false } => "thin",
                    FaVariant { base: FaBaseStyle::Brands, is_sharp: false, is_duotone: false } => "brands",

                    // Sharp variants
                    FaVariant { base: FaBaseStyle::Solid, is_sharp: true, is_duotone: false } => "sharp-solid",
                    FaVariant { base: FaBaseStyle::Regular, is_sharp: true, is_duotone: false } => "sharp-regular",
                    FaVariant { base: FaBaseStyle::Light, is_sharp: true, is_duotone: false } => "sharp-light",
                    FaVariant { base: FaBaseStyle::Thin, is_sharp: true, is_duotone: false } => "sharp-thin",

                    // Duotone variants
                    FaVariant { base: FaBaseStyle::Solid, is_sharp: false, is_duotone: true } => "duotone",
                    FaVariant { base: FaBaseStyle::Regular, is_sharp: false, is_duotone: true } => "duotone-regular",
                    FaVariant { base: FaBaseStyle::Light, is_sharp: false, is_duotone: true } => "duotone-light",
                    FaVariant { base: FaBaseStyle::Thin, is_sharp: false, is_duotone: true } => "duotone-thin",

                    // Sharp duotone variants
                    FaVariant { base: FaBaseStyle::Solid, is_sharp: true, is_duotone: true } => "sharp-duotone-solid",
                    FaVariant { base: FaBaseStyle::Regular, is_sharp: true, is_duotone: true } => "sharp-duotone-regular",
                    FaVariant { base: FaBaseStyle::Light, is_sharp: true, is_duotone: true } => "sharp-duotone-light",
                    FaVariant { base: FaBaseStyle::Thin, is_sharp: true, is_duotone: true } => "sharp-duotone-thin",

                    // Default to solid for any unexpected combinations
                    _ => "solid",
                };

                // Find the appropriate icon file by processing each variant
                process_icon_paths!{self, variant_path, $base_path, [$($variants)*]}
            }
        }
    }
}

#[macro_export]
macro_rules! extract_enum_variants {
    // Simple recursive macro to extract identifiers from variant definitions
    ([], $public:vis) => {
        #[derive(Debug, Clone, Copy)]
        $public enum FaIcon {}
    };

    ([$variant:ident = $path:expr, $($rest:tt)*], $public:vis) => {
        extract_enum_variants!([$($rest)*], $public, [$variant]);
    };

    ([$variant:ident, $($rest:tt)*], $public:vis) => {
        extract_enum_variants!([$($rest)*], $public, [$variant]);
    };

    ([$variant:ident = $path:expr], $public:vis) => {
        extract_enum_variants!([], $public, [$variant]);
    };

    ([$variant:ident], $public:vis) => {
        extract_enum_variants!([], $public, [$variant]);
    };

    ([], $public:vis, [$($variants:ident),*]) => {
        #[derive(Debug, Clone, Copy)]
        $public enum FaIcon {
            $($variants),*
        }
    };

    ([$first:ident = $path:expr, $($rest:tt)*], $public:vis, [$($variants:ident),*]) => {
        extract_enum_variants!([$($rest)*], $public, [$($variants),*, $first]);
    };

    ([$first:ident, $($rest:tt)*], $public:vis, [$($variants:ident),*]) => {
        extract_enum_variants!([$($rest)*], $public, [$($variants),*, $first]);
    };
}

#[macro_export]
macro_rules! process_icon_paths {
    // Base case - no more variants to process
    {$self:expr, $variant_path:expr, $base_path:expr, []} => {
        // Default fallback
        r#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24"></svg>"#
    };

    // Process a variant with a custom path
    {$self:expr, $variant_path:expr, $base_path:expr,
        [$variant:ident = $path:expr, $($rest:tt)*]
    } => {
        match ($self, $variant_path) {
            (FaIcon::$variant, "solid") => include_str!(concat!($base_path, "/solid/", $path, ".svg")),
            (FaIcon::$variant, "regular") => include_str!(concat!($base_path, "/regular/", $path, ".svg")),
            (FaIcon::$variant, "light") => include_str!(concat!($base_path, "/light/", $path, ".svg")),
            (FaIcon::$variant, "thin") => include_str!(concat!($base_path, "/thin/", $path, ".svg")),
            (FaIcon::$variant, "duotone") => include_str!(concat!($base_path, "/duotone/", $path, ".svg")),
            (FaIcon::$variant, "sharp-solid") => include_str!(concat!($base_path, "/sharp-solid/", $path, ".svg")),
            (FaIcon::$variant, "sharp-regular") => include_str!(concat!($base_path, "/sharp-regular/", $path, ".svg")),
            (FaIcon::$variant, "sharp-light") => include_str!(concat!($base_path, "/sharp-light/", $path, ".svg")),
            (FaIcon::$variant, "sharp-thin") => include_str!(concat!($base_path, "/sharp-thin/", $path, ".svg")),
            (FaIcon::$variant, "sharp-duotone-solid") => include_str!(concat!($base_path, "/sharp-duotone-solid/", $path, ".svg")),
            (FaIcon::$variant, "sharp-duotone-regular") => include_str!(concat!($base_path, "/sharp-duotone-regular/", $path, ".svg")),
            (FaIcon::$variant, "sharp-duotone-light") => include_str!(concat!($base_path, "/sharp-duotone-light/", $path, ".svg")),
            (FaIcon::$variant, "sharp-duotone-thin") => include_str!(concat!($base_path, "/sharp-duotone-thin/", $path, ".svg")),
            (FaIcon::$variant, "duotone-regular") => include_str!(concat!($base_path, "/duotone-regular/", $path, ".svg")),
            (FaIcon::$variant, "duotone-light") => include_str!(concat!($base_path, "/duotone-light/", $path, ".svg")),
            (FaIcon::$variant, "duotone-thin") => include_str!(concat!($base_path, "/duotone-thin/", $path, ".svg")),
            (FaIcon::$variant, "brands") => include_str!(concat!($base_path, "/solid/", $path, ".svg")),
            _ => process_icon_paths!{$self, $variant_path, $base_path, [$($rest)*]}
        }
    };

    // Process a regular variant (no custom path)
    {$self:expr, $variant_path:expr, $base_path:expr,
        [$variant:ident, $($rest:tt)*]
    } => {
        match ($self, $variant_path) {
            (FaIcon::$variant, "solid") => include_str!(concat!($base_path, "/solid/", stringify!($variant), ".svg")),
            (FaIcon::$variant, "regular") => include_str!(concat!($base_path, "/regular/", stringify!($variant), ".svg")),
            (FaIcon::$variant, "light") => include_str!(concat!($base_path, "/light/", stringify!($variant), ".svg")),
            (FaIcon::$variant, "thin") => include_str!(concat!($base_path, "/thin/", stringify!($variant), ".svg")),
            (FaIcon::$variant, "duotone") => include_str!(concat!($base_path, "/duotone/", stringify!($variant), ".svg")),
            (FaIcon::$variant, "sharp-solid") => include_str!(concat!($base_path, "/sharp-solid/", stringify!($variant), ".svg")),
            (FaIcon::$variant, "sharp-regular") => include_str!(concat!($base_path, "/sharp-regular/", stringify!($variant), ".svg")),
            (FaIcon::$variant, "sharp-light") => include_str!(concat!($base_path, "/sharp-light/", stringify!($variant), ".svg")),
            (FaIcon::$variant, "sharp-thin") => include_str!(concat!($base_path, "/sharp-thin/", stringify!($variant), ".svg")),
            (FaIcon::$variant, "sharp-duotone-solid") => include_str!(concat!($base_path, "/sharp-duotone-solid/", stringify!($variant), ".svg")),
            (FaIcon::$variant, "sharp-duotone-regular") => include_str!(concat!($base_path, "/sharp-duotone-regular/", stringify!($variant), ".svg")),
            (FaIcon::$variant, "sharp-duotone-light") => include_str!(concat!($base_path, "/sharp-duotone-light/", stringify!($variant), ".svg")),
            (FaIcon::$variant, "sharp-duotone-thin") => include_str!(concat!($base_path, "/sharp-duotone-thin/", stringify!($variant), ".svg")),
            (FaIcon::$variant, "duotone-regular") => include_str!(concat!($base_path, "/duotone-regular/", stringify!($variant), ".svg")),
            (FaIcon::$variant, "duotone-light") => include_str!(concat!($base_path, "/duotone-light/", stringify!($variant), ".svg")),
            (FaIcon::$variant, "duotone-thin") => include_str!(concat!($base_path, "/duotone-thin/", stringify!($variant), ".svg")),
            (FaIcon::$variant, "brands") => include_str!(concat!($base_path, "/solid/", stringify!($variant), ".svg")),
            _ => process_icon_paths!{$self, $variant_path, $base_path, [$($rest)*]}
        }
    };

    // Handle the last variant with a custom path (no comma at the end)
    {$self:expr, $variant_path:expr, $base_path:expr,
        [$variant:ident = $path:expr]
    } => {
        match ($self, $variant_path) {
            (FaIcon::$variant, "solid") => include_str!(concat!($base_path, "/solid/", $path, ".svg")),
            (FaIcon::$variant, "regular") => include_str!(concat!($base_path, "/regular/", $path, ".svg")),
            (FaIcon::$variant, "light") => include_str!(concat!($base_path, "/light/", $path, ".svg")),
            (FaIcon::$variant, "thin") => include_str!(concat!($base_path, "/thin/", $path, ".svg")),
            (FaIcon::$variant, "duotone") => include_str!(concat!($base_path, "/duotone/", $path, ".svg")),
            (FaIcon::$variant, "sharp-solid") => include_str!(concat!($base_path, "/sharp-solid/", $path, ".svg")),
            (FaIcon::$variant, "sharp-regular") => include_str!(concat!($base_path, "/sharp-regular/", $path, ".svg")),
            (FaIcon::$variant, "sharp-light") => include_str!(concat!($base_path, "/sharp-light/", $path, ".svg")),
            (FaIcon::$variant, "sharp-thin") => include_str!(concat!($base_path, "/sharp-thin/", $path, ".svg")),
            (FaIcon::$variant, "sharp-duotone-solid") => include_str!(concat!($base_path, "/sharp-duotone-solid/", $path, ".svg")),
            (FaIcon::$variant, "sharp-duotone-regular") => include_str!(concat!($base_path, "/sharp-duotone-regular/", $path, ".svg")),
            (FaIcon::$variant, "sharp-duotone-light") => include_str!(concat!($base_path, "/sharp-duotone-light/", $path, ".svg")),
            (FaIcon::$variant, "sharp-duotone-thin") => include_str!(concat!($base_path, "/sharp-duotone-thin/", $path, ".svg")),
            (FaIcon::$variant, "duotone-regular") => include_str!(concat!($base_path, "/duotone-regular/", $path, ".svg")),
            (FaIcon::$variant, "duotone-light") => include_str!(concat!($base_path, "/duotone-light/", $path, ".svg")),
            (FaIcon::$variant, "duotone-thin") => include_str!(concat!($base_path, "/duotone-thin/", $path, ".svg")),
            (FaIcon::$variant, "brands") => include_str!(concat!($base_path, "/solid/", $path, ".svg")),
            _ => r#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24"></svg>"#
        }
    };

    // Handle the last regular variant (no comma at the end)
    {$self:expr, $variant_path:expr, $base_path:expr,
        [$variant:ident]
    } => {
        match ($self, $variant_path) {
            (FaIcon::$variant, "solid") => include_str!(concat!($base_path, "/solid/", stringify!($variant), ".svg")),
            (FaIcon::$variant, "regular") => include_str!(concat!($base_path, "/regular/", stringify!($variant), ".svg")),
            (FaIcon::$variant, "light") => include_str!(concat!($base_path, "/light/", stringify!($variant), ".svg")),
            (FaIcon::$variant, "thin") => include_str!(concat!($base_path, "/thin/", stringify!($variant), ".svg")),
            (FaIcon::$variant, "duotone") => include_str!(concat!($base_path, "/duotone/", stringify!($variant), ".svg")),
            (FaIcon::$variant, "sharp-solid") => include_str!(concat!($base_path, "/sharp-solid/", stringify!($variant), ".svg")),
            (FaIcon::$variant, "sharp-regular") => include_str!(concat!($base_path, "/sharp-regular/", stringify!($variant), ".svg")),
            (FaIcon::$variant, "sharp-light") => include_str!(concat!($base_path, "/sharp-light/", stringify!($variant), ".svg")),
            (FaIcon::$variant, "sharp-thin") => include_str!(concat!($base_path, "/sharp-thin/", stringify!($variant), ".svg")),
            (FaIcon::$variant, "sharp-duotone-solid") => include_str!(concat!($base_path, "/sharp-duotone-solid/", stringify!($variant), ".svg")),
            (FaIcon::$variant, "sharp-duotone-regular") => include_str!(concat!($base_path, "/sharp-duotone-regular/", stringify!($variant), ".svg")),
            (FaIcon::$variant, "sharp-duotone-light") => include_str!(concat!($base_path, "/sharp-duotone-light/", stringify!($variant), ".svg")),
            (FaIcon::$variant, "sharp-duotone-thin") => include_str!(concat!($base_path, "/sharp-duotone-thin/", stringify!($variant), ".svg")),
            (FaIcon::$variant, "duotone-regular") => include_str!(concat!($base_path, "/duotone-regular/", stringify!($variant), ".svg")),
            (FaIcon::$variant, "duotone-light") => include_str!(concat!($base_path, "/duotone-light/", stringify!($variant), ".svg")),
            (FaIcon::$variant, "duotone-thin") => include_str!(concat!($base_path, "/duotone-thin/", stringify!($variant), ".svg")),
            (FaIcon::$variant, "brands") => include_str!(concat!($base_path, "/solid/", stringify!($variant), ".svg")),
            _ => r#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24"></svg>"#
        }
    };
}
