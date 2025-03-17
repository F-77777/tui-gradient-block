/// A module of predefined border styles for different visual aesthetics. Each `BorderSymbolSet`
/// instance defines the characters to be used for different parts of the border (corners, sides, and centers).
///
/// # Variants:
/// - `MISC1`: A style with standard "+" corners, and "=" for top/bottom edges, with "|" for side edges.
/// - `MISC2`: A style with "╘" and "╛" for the bottom corners, and "=" for top and bottom edges.
/// - `MISC3`: A style with "╬" for the corners and sides, and "═" for the top and bottom edges.
/// - `MISC4`: A unique style with "$" corners, "~" for center sides, and "─" for top and bottom edges.
///
/// These styles can be used to customize the appearance of borders for blocks
use crate::structs::border_symbols::BorderSymbolsSet;
pub const MISC1: BorderSymbolsSet =
    BorderSymbolsSet {
        top_left: Some('+'),
        bottom_left: Some('+'),
        top_right: Some('+'),
        bottom_right: Some('+'),
        top_horizontal: Some('='),
        bottom_horizontal: Some('='),
        left_vertical: Some('|'),
        right_vertical: Some('|'),
        bottom_center: Some('='),
        top_center: Some('='),
        right_center: Some('|'),
        left_center: Some('|'),

        top_horizontal_right: None,
        bottom_horizontal_right: None,
        top_horizontal_left: None,
        bottom_horizontal_left: None,
        top_vertical_right: None,
        bottom_vertical_right: None,
        top_vertical_left: None,
        bottom_vertical_left: None,
    };
/// A simple border style with "&" edges
pub const MISC2: BorderSymbolsSet =
    BorderSymbolsSet {
        top_left: Some('&'),
        bottom_left: Some('&'),
        top_right: Some('&'),
        bottom_right: Some('&'),
        top_horizontal: Some('-'),
        bottom_horizontal: Some('-'),
        right_center: Some('+'),
        left_center: Some('+'),
        left_vertical: Some('|'),
        right_vertical: Some('|'),
        bottom_center: Some('-'),
        top_center: Some('-'),

        top_horizontal_right: None,
        bottom_horizontal_right: None,
        top_horizontal_left: None,
        bottom_horizontal_left: None,
        top_vertical_right: None,
        bottom_vertical_right: None,
        top_vertical_left: None,
        bottom_vertical_left: None,
    };

/// A border style with "╬" for the corners and sides, giving a more solid and ornate border.
pub const MISC3: BorderSymbolsSet =
    BorderSymbolsSet {
        top_left: Some('╬'),
        bottom_left: Some('╬'),
        top_right: Some('╬'),
        bottom_right: Some('╬'),
        top_horizontal: Some('═'),
        bottom_horizontal: Some('═'),
        left_vertical: Some('║'),
        right_vertical: Some('║'),
        bottom_center: Some('═'),
        top_center: Some('═'),
        right_center: Some('║'),
        left_center: Some('║'),

        top_horizontal_right: None,
        bottom_horizontal_right: None,
        top_horizontal_left: None,
        bottom_horizontal_left: None,
        top_vertical_right: None,
        bottom_vertical_right: None,
        top_vertical_left: None,
        bottom_vertical_left: None,
    };

/// A more unique border style featuring "$" for the corners, "~" for the center sides, and "─" for the edges.
pub const MISC4: BorderSymbolsSet =
    BorderSymbolsSet {
        top_left: Some('$'),
        bottom_left: Some('$'),
        top_right: Some('$'),
        bottom_right: Some('$'),
        top_horizontal: Some('─'),
        bottom_horizontal: Some('─'),
        left_vertical: Some('│'),
        right_vertical: Some('│'),
        bottom_center: Some('$'),
        top_center: Some('~'),
        right_center: Some('~'),
        left_center: Some('~'),

        top_horizontal_right: None,
        bottom_horizontal_right: None,
        top_horizontal_left: None,
        bottom_horizontal_left: None,
        top_vertical_right: None,
        bottom_vertical_right: None,
        top_vertical_left: None,
        bottom_vertical_left: None,
    };
