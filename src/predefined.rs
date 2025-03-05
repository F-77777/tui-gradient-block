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
pub mod border_styles {
    use crate::types::structs::BorderSymbolSet;
    pub const MISC1: BorderSymbolSet = BorderSymbolSet {
        top_left: '+',
        bottom_left: '+',
        top_right: '+',
        bottom_right: '+',
        top: '=',
        bottom: '=',
        left: '|',
        right: '|',
        bottom_center: '=',
        top_center: '=',
        right_center: '|',
        left_center: '|',
    };

    /// A simple border style with "&" edges
    pub const MISC2: BorderSymbolSet = BorderSymbolSet {
        top_left: '&',
        bottom_left: '&',
        top_right: '&',
        bottom_right: '&',
        top: '-',
        bottom: '-',
        right_center: '+',
        left_center: '+',
        left: '|',
        right: '|',
        bottom_center: '-',
        top_center: '-',
    };

    /// A border style with "╬" for the corners and sides, giving a more solid and ornate border.
    pub const MISC3: BorderSymbolSet = BorderSymbolSet {
        top_left: '╬',
        bottom_left: '╬',
        top_right: '╬',
        bottom_right: '╬',
        top: '═',
        bottom: '═',
        left: '║',
        right: '║',
        bottom_center: '═',
        top_center: '═',
        right_center: '║',
        left_center: '║',
    };

    /// A more unique border style featuring "$" for the corners, "~" for the center sides, and "─" for the edges.
    pub const MISC4: BorderSymbolSet = BorderSymbolSet {
        top_left: '$',
        bottom_left: '$',
        top_right: '$',
        bottom_right: '$',
        top: '─',
        bottom: '─',
        left: '│',
        right: '│',
        bottom_center: '$',
        top_center: '~',
        right_center: '~',
        left_center: '~',
    };
}

pub mod consts {

    /// A constant error message displayed when there are not enough colors provided for a gradient.
    /// It encourages the user to use at least two colors for a gradient effect or repeat the same color
    /// if a solid color is desired.
    ///
    /// Example of how to provide colors for a solid color:
    /// ❌ `[(250, 2, 238)]`
    /// ✅ `[(250, 2, 238), (250, 2, 238)]`
    pub const ERROR_MESSAGE: &str = "
╓───── IMPORTANT ─────╖
║                     ║
║ Use at least two    ║
║ colors.             ║
║                     ║
║ If you want to use  ║
║ a solid color,      ║
║ enter the same      ║
║ color more than once║
║                     ║
║ Example: Solid pink ║
║                     ║
║ ❌ [(250, 2, 238)]  ║
║ ✅ [                ║
║     (250, 2, 238),  ║
║     (250, 2, 238),  ║
║    ]                ║
║                     ║
╙─────────────────────╜
";
}
