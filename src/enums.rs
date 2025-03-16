#[derive(Clone, Hash, PartialEq)]
/// Contains miscellaneous border types
pub enum MiscBorderTypes {
    /// ```
    /// +=====+
    /// |     |
    /// |     |
    /// |     |
    /// +=====+
    /// ```
    Misc1,
    /// ```
    /// &-----&
    /// |     |
    /// +     +
    /// |     |
    /// &-----&
    /// ```
    Misc2,
    /// ``````
    /// ╬═════╬
    /// ║     ║
    /// ║     ║
    /// ║     ║
    /// ╬═════╬
    /// ```
    Misc3,
    /// ```
    /// $──~──$
    /// |     |
    /// ~     ~
    /// |     |
    /// $──~──$
    /// ```
    Misc4,
}

#[derive(Clone, PartialEq, Hash)]
/// contains the custom border types
/// Defines different border styles that can be applied.
pub enum BorderStyle {
    /// A simple, single-line border (e.g., `│─┌┐└┘`).
    Plain,
    /// A double-line border for a more structured appearance (e.g., `║═╔╗╚╝`).
    Double,
    /// A thick border for strong emphasis (may vary depending on rendering support).
    Thick,
    /// A rounded border with smooth corners (e.g., `╭╮╰╯`).
    Rounded,
    /// a custom user-defined border type that can be serialized and deserialized
    CustomBorderType(
        crate::structs::border_symbols::BorderSymbolsSetMin,
    ),
    CustomBorderTypeFull(
        crate::structs::border_symbols::BorderSymbolsSet,
    ),
    /// A completely empty, user-defined custom border.
    EmptyBorderType,
    /// A collection of miscellaneous border types.
    MiscBorder(MiscBorderTypes),
}
#[derive(Debug, Clone)]
/// contains the custom border types
/// Defines different border styles that can be applied.
pub enum BorderStyleArgs {
    /// A simple, single-line border (e.g., `│─┌┐└┘`).
    Plain,
    /// A double-line border for a more structured appearance (e.g., `║═╔╗╚╝`).
    Double,
    /// A thick border for strong emphasis (may vary depending on rendering support).
    Thick,
    /// A rounded border with smooth corners (e.g., `╭╮╰╯`).
    Rounded,
}
#[derive(Debug, Clone, PartialEq, Hash, serde::Serialize, serde::Deserialize)]
pub enum GradientType {
    Linear,
    CatmullRom,
    Basis,
}