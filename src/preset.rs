use crate::structs::border_symbols::SegmentSet;
use tui_rule::{Set, presets::neutral::EMPTY as EMPT};
// A module of predefined border styles for different visual aesthetics. Each `SegmentSet`
// instance defines the characters to be used for different parts of the border (corners, sides, and centers).
//
// These styles can be used to customize the appearance of borders for blocks
/// ```
/// &-----&
/// |     |
/// +     +
/// |     |
/// &-----&
/// ```
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
/// ```
/// &-----&
/// |     |
/// +     +
/// |     |
/// &-----&
/// ```
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
/// ```
/// $──~──$
/// |     |
/// ~     ~
/// |     |
/// $──~──$
/// ```
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
pub const EMPTY: SegmentSet = SegmentSet {
    left: EMPT,
    right: EMPT,
    top: EMPT,
    bottom: EMPT,
};
