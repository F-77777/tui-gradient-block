crate::generate_theme_use!();

pub const COLOR_1: C = C::new(32, 25, 71, 1);
pub const COLOR_2: C = C::new(75, 51, 183, 1);
pub const COLOR_3: C = C::new(138, 73, 252, 1);
pub const COLOR_4: C = C::new(145, 82, 255, 1);

pub static TITLES: Lazy<
    Mutex<structs::TitleSet>,
> = Lazy::new(|| {
    Mutex::new(gen_titles!(
        COLOR_2, COLOR_3, COLOR_4
    ))
});

pub const D_TO_L: GRef = GRef {
    colors: &[COLOR_1, COLOR_2],
    gradient_type: GType::Linear,
    gradient_color_count: None,
};

pub const L_TO_D: GRef = GRef {
    colors: &[COLOR_2, COLOR_1],
    gradient_type: GType::Linear,
    gradient_color_count: None,
};

pub const COL_1: GRef = GRef {
    colors: &[COLOR_1],
    gradient_type: GType::Linear,
    gradient_color_count: None,
};

pub const COL_2: GRef = GRef {
    colors: &[COLOR_2],
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
    top: GRef {
        colors: &[COLOR_1, COLOR_1, COLOR_4],
        gradient_type: GType::Linear,
        gradient_color_count: None,
    },
    left: GRef {
        colors: &[
            COLOR_1, COLOR_2, COLOR_2, COLOR_4,
        ],
        gradient_type: GType::Linear,
        gradient_color_count: None,
    },
    bottom: GRef {
        colors: &[COLOR_4, COLOR_1, COLOR_1],
        gradient_type: GType::Linear,
        gradient_color_count: None,
    },
    right: GRef {
        colors: &[
            COLOR_4, COLOR_2, COLOR_1, COLOR_1,
        ],
        gradient_type: GType::Linear,
        gradient_color_count: None,
    },
};

pub const DOUBLE_LEFT: GVRef = GVRef {
    top: GRef {
        colors: &[COLOR_2, COLOR_1, COLOR_1],
        gradient_type: GType::Linear,
        gradient_color_count: None,
    },
    left: GRef {
        colors: &[COLOR_2, COLOR_1],
        gradient_type: GType::Linear,
        gradient_color_count: None,
    },
    bottom: GRef {
        colors: &[COLOR_1, COLOR_1, COLOR_2],
        gradient_type: GType::Linear,
        gradient_color_count: None,
    },
    right: GRef {
        colors: &[COLOR_1, COLOR_2],
        gradient_type: GType::Linear,
        gradient_color_count: None,
    },
};

pub const TOP_LEFT: GVRef = GVRef {
    top: D_TO_L,
    left: D_TO_L,
    bottom: COL_1,
    right: COL_1,
};

pub const TOP_RIGHT: GVRef = GVRef {
    top: L_TO_D,
    left: COL_1,
    bottom: COL_1,
    right: L_TO_D,
};

pub const HORIZONTAL: GVRef = GVRef {
    top: GRef {
        colors: &[COLOR_1, COLOR_2, COLOR_1],
        gradient_type: GType::Linear,
        gradient_color_count: None,
    },
    left: COL_1,
    bottom: GRef {
        colors: &[COLOR_1, COLOR_2, COLOR_1],
        gradient_type: GType::Linear,
        gradient_color_count: None,
    },
    right: COL_1,
};

pub const VERTICAL: GVRef = GVRef {
    top: COL_1,
    left: GRef {
        colors: &[COLOR_1, COLOR_2, COLOR_1],
        gradient_type: GType::Linear,
        gradient_color_count: None,
    },
    bottom: COL_1,
    right: GRef {
        colors: &[COLOR_1, COLOR_2, COLOR_1],
        gradient_type: GType::Linear,
        gradient_color_count: None,
    },
};

pub const UP: GVRef = GVRef {
    top: COL_2,
    left: L_TO_D,
    bottom: COL_1,
    right: L_TO_D,
};

pub const DOWN: GVRef = GVRef {
    top: COL_1,
    left: D_TO_L,
    bottom: COL_2,
    right: D_TO_L,
};

pub const LEFT: GVRef = GVRef {
    top: L_TO_D,
    left: COL_2,
    bottom: L_TO_D,
    right: COL_1,
};

pub const RIGHT: GVRef = GVRef {
    top: D_TO_L,
    left: COL_1,
    bottom: D_TO_L,
    right: COL_2,
};

pub const BASE1: GVRef = GVRef {
    top: GRef {
        colors: &[
            COLOR_4, COLOR_2, COLOR_2, COLOR_2,
            COLOR_4,
        ],
        gradient_type: GType::Linear,
        gradient_color_count: None,
    },
    left: GRef {
        colors: &[
            COLOR_4, COLOR_2, COLOR_2, COLOR_4,
            COLOR_2,
        ],
        gradient_type: GType::Linear,
        gradient_color_count: None,
    },
    bottom: GRef {
        colors: &[COLOR_2, COLOR_4, COLOR_2],
        gradient_type: GType::Linear,
        gradient_color_count: None,
    },
    right: GRef {
        colors: &[COLOR_4, COLOR_2, COLOR_2],
        gradient_type: GType::Linear,
        gradient_color_count: None,
    },
};

pub const BASE2: GVRef = GVRef {
    top: GRef {
        colors: &[
            COLOR_4, COLOR_2, COLOR_2, COLOR_2,
        ],
        gradient_type: GType::Linear,
        gradient_color_count: None,
    },
    left: GRef {
        colors: &[COLOR_4, COLOR_2],
        gradient_type: GType::Linear,
        gradient_color_count: None,
    },
    bottom: GRef {
        colors: &[COLOR_2, COLOR_2, COLOR_4],
        gradient_type: GType::Linear,
        gradient_color_count: None,
    },
    right: GRef {
        colors: &[
            COLOR_2, COLOR_3, COLOR_2, COLOR_4,
        ],
        gradient_type: GType::Linear,
        gradient_color_count: None,
    },
};
