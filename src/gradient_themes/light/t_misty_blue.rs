crate::generate_theme_use!();

pub const COLOR_1: C = C::new(28, 123, 186, 1);
pub const COLOR_2: C = C::new(48, 140, 197, 1);
pub const COLOR_3: C = C::new(74, 156, 207, 1);
pub const COLOR_4: C = C::new(118, 179, 214, 1);
pub const COLOR_5: C = C::new(189, 215, 234, 1);
pub static TITLES: Lazy<
    Mutex<structs::TitleSet>,
> = Lazy::new(|| {
    Mutex::new(gen_titles!(
        COLOR_2, COLOR_3, COLOR_4
    ))
});
pub const VERTICAL_G: GRef = GRef {
    colors: &[
        COLOR_1, COLOR_2, COLOR_3, COLOR_4,
        COLOR_5, COLOR_4, COLOR_3, COLOR_2,
        COLOR_1,
    ],
    gradient_type: GType::CatmullRom,
    gradient_color_count: None,
};

pub const HORIZONTAL_G: GRef = GRef {
    colors: &[
        COLOR_1, COLOR_2, COLOR_3, COLOR_4,
        COLOR_4, COLOR_4, COLOR_4, COLOR_4,
        COLOR_3, COLOR_2, COLOR_1,
    ],
    gradient_type: GType::CatmullRom,
    gradient_color_count: None,
};

pub const D_TO_L: GRef = GRef {
    colors: &[
        COLOR_1, COLOR_2, COLOR_3, COLOR_4,
        COLOR_4,
    ],
    gradient_type: GType::Linear,
    gradient_color_count: None,
};

pub const L_TO_D: GRef = GRef {
    colors: &[
        COLOR_4, COLOR_4, COLOR_3, COLOR_2,
        COLOR_1,
    ],
    gradient_type: GType::Linear,
    gradient_color_count: None,
};

pub const COL_1: GRef = GRef {
    colors: &[COLOR_1],
    gradient_type: GType::Linear,
    gradient_color_count: None,
};

pub const COL_5: GRef = GRef {
    colors: &[COLOR_5],
    gradient_type: GType::Linear,
    gradient_color_count: None,
};

pub const BOTTOM_RIGHT: GVRef = GVRef {
    top: COL_1,
    left: COL_1,
    bottom: D_TO_L,
    right: D_TO_L,
};
pub const BOTTOM_LEFT: GVRef = GVRef {
    top: COL_1,
    left: D_TO_L,
    bottom: L_TO_D,
    right: COL_1,
};
pub const DOUBLE_RIGHT: GVRef = GVRef {
    top: D_TO_L,
    left: D_TO_L,
    bottom: L_TO_D,
    right: L_TO_D,
};
pub const DOUBLE_LEFT: GVRef = GVRef {
    top: L_TO_D,
    left: L_TO_D,
    bottom: D_TO_L,
    right: D_TO_L,
};
pub const TOP_LEFT: GVRef = GVRef {
    top: L_TO_D,
    left: L_TO_D,
    bottom: COL_1,
    right: COL_1,
};
pub const TOP_RIGHT: GVRef = GVRef {
    top: D_TO_L,
    left: COL_1,
    bottom: COL_1,
    right: L_TO_D,
};
pub const HORIZONTAL: GVRef = GVRef {
    top: HORIZONTAL_G,
    left: COL_1,
    bottom: HORIZONTAL_G,
    right: COL_1,
};
pub const VERTICAL: GVRef = GVRef {
    top: COL_1,
    left: VERTICAL_G,
    bottom: COL_1,
    right: VERTICAL_G,
};
pub const UP: GVRef = GVRef {
    top: COL_5,
    left: L_TO_D,
    bottom: COL_1,
    right: L_TO_D,
};
pub const DOWN: GVRef = GVRef {
    top: COL_1,
    left: D_TO_L,
    bottom: COL_5,
    right: D_TO_L,
};
pub const LEFT: GVRef = GVRef {
    top: L_TO_D,
    left: COL_5,
    bottom: L_TO_D,
    right: COL_1,
};
pub const RIGHT: GVRef = GVRef {
    top: D_TO_L,
    left: COL_1,
    bottom: D_TO_L,
    right: COL_5,
};
pub const BASE1: GVRef = GVRef {
    top: GRef {
        colors: &[
            COLOR_5, COLOR_2, COLOR_3, COLOR_4,
        ],
        gradient_type: GType::Linear,
        gradient_color_count: None,
    },
    left: GRef {
        colors: &[
            COLOR_5, COLOR_3, COLOR_5, COLOR_2,
        ],
        gradient_type: GType::Linear,
        gradient_color_count: None,
    },
    bottom: GRef {
        colors: &[
            COLOR_2, COLOR_3, COLOR_4, COLOR_3,
            COLOR_2,
        ],
        gradient_type: GType::Linear,
        gradient_color_count: None,
    },
    right: GRef {
        colors: &[COLOR_4, COLOR_3, COLOR_2],
        gradient_type: GType::Linear,
        gradient_color_count: None,
    },
};
pub const BASE2: GVRef = GVRef {
    top: GRef {
        colors: &[COLOR_5, COLOR_3, COLOR_2],
        gradient_type: GType::Linear,
        gradient_color_count: None,
    },
    left: GRef {
        colors: &[COLOR_5, COLOR_2],
        gradient_type: GType::Linear,
        gradient_color_count: None,
    },
    bottom: GRef {
        colors: &[COLOR_2, COLOR_2, COLOR_5],
        gradient_type: GType::Linear,
        gradient_color_count: None,
    },
    right: GRef {
        colors: &[
            COLOR_2, COLOR_5, COLOR_2, COLOR_5,
        ],
        gradient_type: GType::Linear,
        gradient_color_count: None,
    },
};
