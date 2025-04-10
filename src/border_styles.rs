use crate::structs::border_symbols::SegmentSet;
use tui_rule::Set;
// A module of predefined border styles for different visual aesthetics. Each `BorderSymbolSet`
// instance defines the characters to be used for different parts of the border (corners, sides, and centers).
//
// # Variants:
// - `MISC1`: A style with standard "+" corners, and "=" for top/bottom edges, with "|" for side edges.
// - `MISC2`: A style with "╘" and "╛" for the bottom corners, and "=" for top and bottom edges.
// - `MISC3`: A unique style with "$" corners, "~" for center sides, and "─" for top and bottom edges.
// These styles can be used to customize the appearance of borders for blocks
pub const MISC1: SegmentSet = SegmentSet {
    left: Set {
        start: '+',
        rep_1: '|',
        center: '+',
        rep_2: '|',
        end: '+',
    },
    right: Set {
        start: '+',
        rep_1: '|',
        center: '+',
        rep_2: '|',
        end: '+',
    },
    top: Set {
        start: '+',
        rep_1: '-',
        center: '+',
        rep_2: '-',
        end: '+',
    },
    bottom: Set {
        start: '+',
        rep_1: '-',
        center: '+',
        rep_2: '-',
        end: '+',
    },
};
/// A simple border style with "&" edges and "+" center symbols
pub const MISC2: SegmentSet = SegmentSet {
    left: Set {
        start: '&',
        rep_1: '|',
        center: '+',
        rep_2: '|',
        end: '&',
    },
    right: Set {
        start: '&',
        rep_1: '|',
        center: '+',
        rep_2: '|',
        end: '&',
    },
    top: Set {
        start: '&',
        rep_1: '-',
        center: '-',
        rep_2: '-',
        end: '&',
    },
    bottom: Set {
        start: '&',
        rep_1: '-',
        center: '-',
        rep_2: '-',
        end: '&',
    },
};

/// A more unique border style featuring "$" for the corners and "~" for the center sides
pub const MISC3: SegmentSet = SegmentSet {
    left: Set {
        start: '$',
        rep_1: '│',
        center: '~',
        rep_2: '│',
        end: '$',
    },
    right: Set {
        start: '$',
        rep_1: '│',
        center: '~',
        rep_2: '│',
        end: '$',
    },
    top: Set {
        start: '$',
        rep_1: '─',
        center: '~',
        rep_2: '─',
        end: '$',
    },
    bottom: Set {
        start: '$',
        rep_1: '─',
        center: '$',
        rep_2: '─',
        end: '$',
    },
};
