pub use crate::{
    types::{
        enums::GradientSegments,
        structs::{BorderGradients, Gradient, GradientThemeSet},
    },
    *,
};

const COLOR_1: (u8, u8, u8) = (28, 123, 186);
const COLOR_2: (u8, u8, u8) = (48, 140, 197);
const COLOR_3: (u8, u8, u8) = (74, 156, 207);
const COLOR_4: (u8, u8, u8) = (118, 179, 214);
const COLOR_5: (u8, u8, u8) = (189, 215, 234);
lazy_static! {
    pub static ref MISTY_BLUE_TITLES: TitleSet =
        gen_titles!(COLOR_3, COLOR_4, COLOR_5);
}
lazy_static! {
    pub static ref MISTY_BLUE: GradientThemeSet = GradientThemeSet {
        bottom_right: generate_gradient_theme!(BorderGradients {
            top: vec![COLOR_1, COLOR_1],
            top_fac: 1.0,
            left: vec![COLOR_1, COLOR_1],
            left_fac: 1.0,
            bottom: vec![COLOR_1, COLOR_2, COLOR_3, COLOR_4, COLOR_5],
            bottom_fac: 1.0,
            right: vec![COLOR_1, COLOR_2, COLOR_3, COLOR_4, COLOR_5],
            right_fac: 1.0,
        }),
        bottom_left: generate_gradient_theme!(BorderGradients {
            top: vec![COLOR_1, COLOR_1],
            top_fac: 1.0,
            left: vec![COLOR_1, COLOR_2, COLOR_3, COLOR_4, COLOR_5],
            left_fac: 1.0,
            bottom: vec![COLOR_5, COLOR_4, COLOR_3, COLOR_2, COLOR_1],
            bottom_fac: 1.0,
            right: vec![COLOR_1, COLOR_1],
            right_fac: 1.0,
        }),
        double_right: generate_gradient_theme!(BorderGradients {
            top: vec![
                COLOR_1, COLOR_1, COLOR_2, COLOR_3, COLOR_4, COLOR_5
            ],
            top_fac: 1.0,
            left: vec![COLOR_1, COLOR_2, COLOR_3, COLOR_4, COLOR_5],
            left_fac: 1.0,
            bottom: vec![COLOR_5, COLOR_4, COLOR_3, COLOR_2, COLOR_1],
            bottom_fac: 1.0,
            right: vec![COLOR_5, COLOR_4, COLOR_3, COLOR_2, COLOR_1],
            right_fac: 1.0,
        }),
        double_left: generate_gradient_theme!(BorderGradients {
            top: vec![COLOR_5, COLOR_4, COLOR_3, COLOR_2, COLOR_1],
            top_fac: 1.0,
            left: vec![COLOR_5, COLOR_4, COLOR_3, COLOR_2, COLOR_1],
            left_fac: 1.0,
            bottom: vec![COLOR_1, COLOR_2, COLOR_3, COLOR_4, COLOR_5],
            bottom_fac: 1.0,
            right: vec![COLOR_1, COLOR_2, COLOR_3, COLOR_4, COLOR_5],
            right_fac: 1.0,
        }),
        top_left: generate_gradient_theme!(BorderGradients {
            top: vec![COLOR_5, COLOR_4, COLOR_3, COLOR_2, COLOR_1],
            top_fac: 1.0,
            left: vec![COLOR_5, COLOR_4, COLOR_3, COLOR_2, COLOR_1],
            left_fac: 1.0,
            bottom: vec![COLOR_1, COLOR_1],
            bottom_fac: 1.0,
            right: vec![COLOR_1, COLOR_1],
            right_fac: 1.0,
        }),
        top_right: generate_gradient_theme!(BorderGradients {
            top: vec![COLOR_1, COLOR_2, COLOR_3, COLOR_4, COLOR_5],
            top_fac: 1.0,
            left: vec![COLOR_1, COLOR_1],
            left_fac: 1.0,
            bottom: vec![COLOR_1, COLOR_1],
            bottom_fac: 1.0,
            right: vec![COLOR_5, COLOR_4, COLOR_3, COLOR_2, COLOR_1],
            right_fac: 1.0,
        }),
        double_horizontal: generate_gradient_theme!(
            BorderGradients {
                top: vec![
                    COLOR_1, COLOR_2, COLOR_3, COLOR_4, COLOR_5,
                    COLOR_5, COLOR_5, COLOR_4, COLOR_3, COLOR_2,
                    COLOR_1
                ],
                top_fac: 1.0,
                left: vec![
                    get_transformed_rgb!(COLOR_1, 1.1, 1),
                    get_transformed_rgb!(COLOR_1, 1.1, 1),
                ],
                left_fac: 1.0,
                bottom: vec![
                    COLOR_1, COLOR_2, COLOR_3, COLOR_4, COLOR_5,
                    COLOR_5, COLOR_5, COLOR_4, COLOR_3, COLOR_2,
                    COLOR_1
                ],
                bottom_fac: 1.0,
                right: vec![COLOR_1, COLOR_1],
                right_fac: 1.0,
            }
        ),
        double_vertical: generate_gradient_theme!(BorderGradients {
            top: vec![COLOR_1, COLOR_1],
            top_fac: 1.0,
            left: vec![
                COLOR_1, COLOR_2, COLOR_3, COLOR_4, COLOR_5, COLOR_4,
                COLOR_3, COLOR_2, COLOR_1
            ],
            left_fac: 1.0,
            bottom: vec![COLOR_1, COLOR_1],
            bottom_fac: 1.0,
            right: vec![
                COLOR_1, COLOR_2, COLOR_3, COLOR_4, COLOR_5, COLOR_4,
                COLOR_3, COLOR_2, COLOR_1
            ],
            right_fac: 1.0,
        }),
        up: generate_gradient_theme!(BorderGradients {
            top: vec![COLOR_5, COLOR_5],
            top_fac: 1.0,
            left: vec![COLOR_5, COLOR_4, COLOR_3, COLOR_2, COLOR_1],
            left_fac: 1.0,
            bottom: vec![COLOR_1, COLOR_1],
            bottom_fac: 1.0,
            right: vec![COLOR_5, COLOR_4, COLOR_3, COLOR_2, COLOR_1],
            right_fac: 1.0,
        }),
        down: generate_gradient_theme!(BorderGradients {
            top: vec![COLOR_1, COLOR_1],
            top_fac: 1.0,
            left: vec![COLOR_1, COLOR_2, COLOR_3, COLOR_4, COLOR_5],
            left_fac: 1.0,
            bottom: vec![COLOR_5, COLOR_5],
            bottom_fac: 1.0,
            right: vec![COLOR_1, COLOR_2, COLOR_3, COLOR_4, COLOR_5],
            right_fac: 1.0,
        }),
        left: generate_gradient_theme!(BorderGradients {
            top: vec![COLOR_5, COLOR_4, COLOR_3, COLOR_2, COLOR_1],
            top_fac: 1.0,
            left: vec![COLOR_5, COLOR_5],
            left_fac: 1.0,
            bottom: vec![COLOR_5, COLOR_4, COLOR_3, COLOR_2, COLOR_1],
            bottom_fac: 1.0,
            right: vec![COLOR_1, COLOR_1],
            right_fac: 1.0,
        }),

        right: generate_gradient_theme!(BorderGradients {
            top: vec![COLOR_1, COLOR_2, COLOR_3, COLOR_4, COLOR_5],
            top_fac: 1.0,
            left: vec![COLOR_1, COLOR_1],
            left_fac: 1.0,
            bottom: vec![COLOR_1, COLOR_2, COLOR_3, COLOR_4, COLOR_5],
            bottom_fac: 1.0,
            right: vec![COLOR_5, COLOR_5],
            right_fac: 1.0,
        }),
        base1: generate_gradient_theme!(BorderGradients {
            top: vec![COLOR_5, COLOR_2, COLOR_3, COLOR_4],
            top_fac: 1.0,
            left: vec![COLOR_5, COLOR_3, COLOR_5, COLOR_2],
            left_fac: 1.0,
            bottom: vec![COLOR_2, COLOR_3, COLOR_4, COLOR_3, COLOR_2],
            bottom_fac: 1.0,
            right: vec![COLOR_4, COLOR_3, COLOR_2],
            right_fac: 1.0,
        }),
        base2: generate_gradient_theme!(BorderGradients {
            top: vec![COLOR_5, COLOR_3, COLOR_2],
            top_fac: 1.0,
            left: vec![COLOR_5, COLOR_2],
            left_fac: 1.0,
            bottom: vec![COLOR_2, COLOR_2, COLOR_5],
            bottom_fac: 1.0,
            right: vec![COLOR_2, COLOR_5, COLOR_2, COLOR_5],
            right_fac: 1.0,
        }),
    };
}
