crate::generate_theme_use!();
pub const COLOR_1: Color = Color {
    r: 32_f32 / 255.0,
    g: 25_f32 / 255.0,
    b: 71_f32 / 255.0,
    a: 1.0,
};
pub const COLOR_2: Color = Color {
    r: 75_f32 / 255.0,
    g: 51_f32 / 255.0,
    b: 183_f32 / 255.0,
    a: 1.0,
};

pub const COLOR_3: Color = Color {
    r: 145_f32 / 255.0,
    g: 82_f32 / 255.0,
    b: 1.0,
    a: 1.0,
};
pub const COLOR_4: Color = Color {
    r: 138_f32 / 255.0,
    g: 73_f32 / 255.0,
    b: 252_f32 / 255.0,
    a: 1.0,
};

pub fn titles() -> TitleSet<'static> {
    gen_titles!(COLOR_2)
}
pub fn solid(col_num: i32) -> G {
    Box::new(
        GradientBuilder::new()
            .colors(&[match col_num {
                1 => COLOR_1,
                2 => COLOR_2,
                3 => COLOR_3,
                _ => COLOR_1,
            }])
            .build::<colorgrad::LinearGradient>()
            .unwrap(),
    )
}
pub fn d_to_l() -> G {
    Box::new(
        GradientBuilder::new()
            .colors(&[COLOR_1, COLOR_2, COLOR_3])
            .build::<colorgrad::LinearGradient>()
            .unwrap(),
    )
}
pub fn l_to_d() -> G {
    Box::new(
        GradientBuilder::new()
            .colors(&[COLOR_3, COLOR_2, COLOR_1])
            .build::<colorgrad::LinearGradient>()
            .unwrap(),
    )
}
pub fn d_to_l_d() -> G {
    Box::new(
        GradientBuilder::new()
            .colors(&[COLOR_1, COLOR_1, COLOR_2, COLOR_3, COLOR_4])
            .build::<colorgrad::LinearGradient>()
            .unwrap(),
    )
}
pub fn l_to_d_d() -> G {
    Box::new(
        GradientBuilder::new()
            .colors(&[COLOR_3, COLOR_2, COLOR_1, COLOR_1])
            .build::<colorgrad::LinearGradient>()
            .unwrap(),
    )
}
pub fn horizontal_g() -> G {
    Box::new(
        GradientBuilder::new()
            .colors(&[
                COLOR_1, COLOR_3, COLOR_1,
            ])
            .build::<colorgrad::LinearGradient>()
            .unwrap(),
    )
}
pub fn vertical_g() -> G {
    Box::new(
        GradientBuilder::new()
            .colors(&[COLOR_1, COLOR_2, COLOR_3, COLOR_2, COLOR_1])
            .build::<colorgrad::LinearGradient>()
            .unwrap(),
    )
}
pub fn bottom_right() -> GV {
    GV {
        top: solid(1),
        left: solid(1),
        right: d_to_l(),
        bottom: d_to_l(),
    }
}
pub fn bottom_left() -> GV {
    GV {
        top: solid(1),
        right: solid(1),
        left: d_to_l(),
        bottom: l_to_d(),
    }
}
pub fn top_left() -> GV {
    GV {
        top: l_to_d(),
        left: l_to_d(),
        bottom: solid(1),
        right: solid(1),
    }
}
pub fn top_right() -> GV {
    GV {
        top: d_to_l(),
        right: l_to_d(),
        bottom: solid(1),
        left: solid(1),
    }
}
pub fn up() -> GV {
    GV {
        top: solid(3),
        right: l_to_d(),
        left: l_to_d(),
        bottom: solid(1),
    }
}
pub fn down() -> GV {
    GV {
        top: solid(1),
        right: d_to_l(),
        left: d_to_l(),
        bottom: solid(3),
    }
}
pub fn right() -> GV {
    GV {
        top: d_to_l(),
        right: solid(3),
        left: solid(1),
        bottom: d_to_l(),
    }
}
pub fn left() -> GV {
    GV {
        top: l_to_d(),
        right: solid(1),
        left: solid(3),
        bottom: l_to_d(),
    }
}
pub fn horizontal() -> GV {
    GV {
        top: horizontal_g(),
        bottom: horizontal_g(),
        left: solid(1),
        right: solid(1),
    }
}
pub fn vertical() -> GV {
    GV {
        top: solid(1),
        bottom: solid(1),
        left: vertical_g(),
        right: vertical_g(),
    }
}
pub fn double_corners_right() -> GV {
    GV {
        right: l_to_d_d(),
        left: d_to_l_d(),
        top: d_to_l(),
        bottom: l_to_d(),
    }
}
pub fn double_corners_left() -> GV {
    GV {
        right: d_to_l_d(),
        left: l_to_d_d(),
        bottom: d_to_l(),
        top: l_to_d(),
    }
}
pub fn misc1() -> GV {
    GV {
        right: Box::new(
            GradientBuilder::new()
                .colors(&[COLOR_3, COLOR_2, COLOR_2])
                .build::<colorgrad::LinearGradient>()
                .unwrap(),
        ),
        left: Box::new(
            GradientBuilder::new()
                .colors(&[
                    COLOR_3, COLOR_2, COLOR_2, COLOR_3, COLOR_2,
                ])
                .build::<colorgrad::LinearGradient>()
                .unwrap(),
        ),
        bottom: Box::new(
            GradientBuilder::new()
                .colors(&[COLOR_2, COLOR_3, COLOR_2])
                .build::<colorgrad::LinearGradient>()
                .unwrap(),
        ),
        top: Box::new(
            GradientBuilder::new()
                .colors(&[
                    COLOR_3, COLOR_2, COLOR_2, COLOR_2, COLOR_3,
                ])
                .build::<colorgrad::LinearGradient>()
                .unwrap(),
        ),
    }
}
pub fn misc2() -> GV {
    GV {
        right: Box::new(
            GradientBuilder::new()
                .colors(&[COLOR_2, COLOR_4, COLOR_2, COLOR_3])
                .build::<colorgrad::LinearGradient>()
                .unwrap(),
        ),
        left: Box::new(
            GradientBuilder::new()
                .colors(&[COLOR_3, COLOR_2])
                .build::<colorgrad::LinearGradient>()
                .unwrap(),
        ),
        bottom: Box::new(
            GradientBuilder::new()
                .colors(&[COLOR_2, COLOR_2, COLOR_3])
                .build::<colorgrad::LinearGradient>()
                .unwrap(),
        ),
        top: Box::new(
            GradientBuilder::new()
                .colors(&[COLOR_3, COLOR_2, COLOR_2, COLOR_2])
                .build::<colorgrad::LinearGradient>()
                .unwrap(),
        ),
    }
}

pub fn full() -> GT {
    GT {
        top_left: top_left(),
        top_right: top_right(),
        bottom_left: bottom_left(),
        bottom_right: bottom_right(),
        double_corners_right: double_corners_right(),
        double_corners_left: double_corners_left(),
        vertical: vertical(),
        horizontal: horizontal(),
        up: up(),
        down: down(),
        left: left(),
        right: right(),
        misc1: misc1(),
        misc2: misc2(),
    }
}
