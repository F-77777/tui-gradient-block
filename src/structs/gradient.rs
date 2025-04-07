use crate::types::G;
pub struct GradientTheme {
    pub top_left: GradientVariation,
    pub top_right: GradientVariation,
    pub bottom_left: GradientVariation,
    pub bottom_right: GradientVariation,
    pub double_corners_right: GradientVariation,
    pub double_corners_left: GradientVariation,
    pub vertical: GradientVariation,
    pub horizontal: GradientVariation,
    pub up: GradientVariation,
    pub down: GradientVariation,
    pub left: GradientVariation,
    pub right: GradientVariation,
    pub misc1: GradientVariation,
    pub misc2: GradientVariation,
}
pub struct GradientVariation {
    pub left: G,
    pub right: G,
    pub bottom: G,
    pub top: G,
}
