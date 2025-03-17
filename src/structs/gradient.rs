use crate::{enums::GradientType, types::G};
pub struct GradientTheme {
    pub top_left: DeserializableGradientVariation,
    pub top_right:
        DeserializableGradientVariation,
    pub bottom_left:
        DeserializableGradientVariation,
    pub bottom_right:
        DeserializableGradientVariation,
    pub double_right:
        DeserializableGradientVariation,
    pub double_left:
        DeserializableGradientVariation,
    pub vertical: DeserializableGradientVariation,
    pub horizontal:
        DeserializableGradientVariation,
    pub up: DeserializableGradientVariation,
    pub down: DeserializableGradientVariation,
    pub left: DeserializableGradientVariation,
    pub right: DeserializableGradientVariation,
    pub base1: DeserializableGradientVariation,
    pub base2: DeserializableGradientVariation,
}
/// the struct for gradient variations
/// using them needs deserialization of the struct
#[derive(
    Clone,
    Debug,
    PartialEq,
    Hash,
    serde::Serialize,
    serde::Deserialize,
)]
pub struct DeserializableGradientVariation {
    left: &'static str,
    right: &'static str,
    bottom: &'static str,
    top: &'static str,
}
pub struct GradientVariation {
    left: G,
    right: G,
    bottom: G,
    top: G,
}
#[derive(
    Debug,
    serde::Serialize,
    serde::Deserialize,
    Clone,
)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}
pub struct Gradient<'a> {
    pub colors: &'a [colorgrad::Color],
    pub gradient_type: GradientType,
    pub gradient_color_count: Option<usize>,
}
#[derive(
    serde::Serialize,
    serde::Deserialize,
    Clone,
    PartialEq,
)]
pub struct OwnedGradient {
    pub colors: Vec<colorgrad::Color>,
    pub gradient_type: GradientType,
    pub gradient_color_count: Option<usize>,
}
impl Gradient<'_> {
    pub fn to_owned(self) -> OwnedGradient {
        OwnedGradient {
            colors: self.colors.to_vec(),
            gradient_type: self.gradient_type,
            gradient_color_count: self
                .gradient_color_count,
        }
    }
}
impl DeserializableGradientVariation {
    pub fn new(
        left: &'static str,
        right: &'static str,
        bottom: &'static str,
        top: &'static str,
    ) -> Self {
        Self {
            left,
            right,
            bottom,
            top,
        }
    }
    pub fn to_gradient_variation(
        &self,
    ) -> GradientVariation {
        macro_rules! to_gradient {
            ($gradient:expr, $gtype:ty) => {
                Box::new(
                    colorgrad::GradientBuilder::new().colors($gradient).build::<$gtype>().unwrap()
                )
            }
        }
        macro_rules! match_gradient_type {
            ($gradient:expr) => {
                match $gradient.gradient_type {
                    GradientType::Linear => {
                        to_gradient!(&$gradient.colors, colorgrad::LinearGradient)
                    }
                    GradientType::Basis => {
                        to_gradient!(&$gradient.colors, colorgrad::BasisGradient)
                    }
                    GradientType::CatmullRom => {
                        to_gradient!(&$gradient.colors, colorgrad::CatmullRomGradient)
                    }
                }
            };
        }
        let right: OwnedGradient =
            serde_json::from_str(self.right)
                .unwrap();
        let left: OwnedGradient =
            serde_json::from_str(self.left)
                .unwrap();
        let bottom: OwnedGradient =
            serde_json::from_str(self.bottom)
                .unwrap();
        let top: OwnedGradient =
            serde_json::from_str(self.top)
                .unwrap();
        GradientVariation {
            left: match_gradient_type!(left),
            right: match_gradient_type!(right),
            bottom: match_gradient_type!(bottom),
            top: match_gradient_type!(top),
        }
    }
}
