crate::generate_theme_use!();
pub const COLOR_1: Color = Color {
    r: 0_f32 / 255.0,
    g: 158_f32 / 255.0,
    b: 119_f32 / 255.0,
    a: 1.0,
};

pub const COLOR_2: Color = Color {
    r: 45_f32 / 255.0,
    g: 174_f32 / 255.0,
    b: 142_f32 / 255.0,
    a: 1.0,
};

pub const COLOR_3: Color = Color {
    r: 77_f32 / 255.0,
    g: 199_f32 / 255.0,
    b: 166_f32 / 255.0,
    a: 1.0,
};

pub const COLOR_4: Color = Color {
    r: 111_f32 / 255.0,
    g: 216_f32 / 255.0,
    b: 179_f32 / 255.0,
    a: 1.0,
};

pub const COLOR_5: Color = Color {
    r: 168_f32 / 255.0,
    g: 225_f32 / 255.0,
    b: 212_f32 / 255.0,
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
                4 => COLOR_4,
                5 => COLOR_5,
                _ => COLOR_1,
            }])
            .build::<colorgrad::LinearGradient>()
            .unwrap(),
    )
}
pub fn d_to_l() -> G {
    Box::new(
        GradientBuilder::new()
            .colors(&[COLOR_1, COLOR_2, COLOR_3, COLOR_4, COLOR_5])
            .build::<colorgrad::LinearGradient>()
            .unwrap(),
    )
}
pub fn l_to_d() -> G {
    Box::new(
        GradientBuilder::new()
            .colors(&[COLOR_5, COLOR_4, COLOR_3, COLOR_2, COLOR_1])
            .build::<colorgrad::LinearGradient>()
            .unwrap(),
    )
}
pub fn d_to_l_d() -> G {
    Box::new(
        GradientBuilder::new()
            .colors(&[
                COLOR_1, COLOR_1, COLOR_2, COLOR_3, COLOR_4, COLOR_5,
            ])
            .build::<colorgrad::LinearGradient>()
            .unwrap(),
    )
}
pub fn l_to_d_d() -> G {
    Box::new(
        GradientBuilder::new()
            .colors(&[
                COLOR_5, COLOR_4, COLOR_3, COLOR_2, COLOR_1, COLOR_1,
            ])
            .build::<colorgrad::LinearGradient>()
            .unwrap(),
    )
}
pub fn horizontal_g() -> G {
    Box::new(
        GradientBuilder::new()
            .colors(&[
                COLOR_1, COLOR_2, COLOR_3, COLOR_4, COLOR_5, COLOR_5,
                COLOR_4, COLOR_3, COLOR_2, COLOR_1,
            ])
            .build::<colorgrad::LinearGradient>()
            .unwrap(),
    )
}
pub fn vertical_g() -> G {
    Box::new(
        GradientBuilder::new()
            .colors(&[
                COLOR_1, COLOR_2, COLOR_4, COLOR_5, COLOR_4, COLOR_2,
                COLOR_1,
            ])
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
        top: solid(5),
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
        bottom: solid(5),
    }
}
pub fn right() -> GV {
    GV {
        top: d_to_l(),
        right: solid(5),
        left: solid(1),
        bottom: d_to_l(),
    }
}
pub fn left() -> GV {
    GV {
        top: l_to_d(),
        right: solid(1),
        left: solid(5),
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
                .colors(&[COLOR_4, COLOR_3, COLOR_2])
                .build::<colorgrad::LinearGradient>()
                .unwrap(),
        ),
        left: Box::new(
            GradientBuilder::new()
                .colors(&[COLOR_5, COLOR_3, COLOR_5, COLOR_2])
                .build::<colorgrad::LinearGradient>()
                .unwrap(),
        ),
        bottom: Box::new(
            GradientBuilder::new()
                .colors(&[
                    COLOR_2, COLOR_3, COLOR_4, COLOR_3, COLOR_2,
                ])
                .build::<colorgrad::LinearGradient>()
                .unwrap(),
        ),
        top: Box::new(
            GradientBuilder::new()
                .colors(&[COLOR_5, COLOR_3, COLOR_2, COLOR_4])
                .build::<colorgrad::LinearGradient>()
                .unwrap(),
        ),
    }
}
pub fn misc2() -> GV {
    GV {
        right: Box::new(
            GradientBuilder::new()
                .colors(&[COLOR_2, COLOR_1, COLOR_2])
                .build::<colorgrad::LinearGradient>()
                .unwrap(),
        ),
        left: Box::new(
            GradientBuilder::new()
                .colors(&[COLOR_3, COLOR_2])
                .build::<colorgrad::LinearGradient>()
                .unwrap(),
        ),
        bottom: solid(2),
        top: Box::new(
            GradientBuilder::new()
                .colors(&[COLOR_3, COLOR_2])
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
