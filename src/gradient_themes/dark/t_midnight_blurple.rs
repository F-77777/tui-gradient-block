pub use crate::{
    types::{
        enums::GradientSegments,
        structs::{
            BorderGradients, Gradient, GradientThemeSet, TitleSet,
        },
    },
    *,
};
const COLOR_1: (u8, u8, u8) = (32, 25, 71);
const COLOR_2: (u8, u8, u8) = (75, 51, 183);
const COLOR_3: (u8, u8, u8) = (145, 82, 255);
lazy_static! {
    pub static ref MIDNIGHT_BLURPLE_TITLES: TitleSet =
        gen_titles!(COLOR_2, COLOR_3);
}
lazy_static! {
    pub static ref MIDNIGHT_BLURPLE: GradientThemeSet =
        GradientThemeSet {
            bottom_right: generate_gradient_theme!(BorderGradients {
                top: vec![COLOR_1, COLOR_1],
                top_fac: 1.0,
                left: vec![COLOR_1, COLOR_1],
                left_fac: 1.0,
                bottom: vec![COLOR_1, COLOR_2],
                bottom_fac: 1.0,
                right: vec![COLOR_1, COLOR_2],
                right_fac: 1.0,
            }),
            bottom_left: generate_gradient_theme!(BorderGradients {
                top: vec![COLOR_1, COLOR_1],
                top_fac: 1.0,
                left: vec![COLOR_1, COLOR_2],
                left_fac: 1.0,
                bottom: vec![COLOR_2, COLOR_1],
                bottom_fac: 1.0,
                right: vec![COLOR_1, COLOR_1],
                right_fac: 1.0,
            }),
            double_right: generate_gradient_theme!(BorderGradients {
                top: vec![COLOR_1, COLOR_1, COLOR_3],
                top_fac: 1.0,
                left: vec![COLOR_1, COLOR_2, COLOR_2, COLOR_3],
                left_fac: 1.0,
                bottom: vec![COLOR_3, COLOR_1, COLOR_1],
                bottom_fac: 1.0,
                right: vec![COLOR_3, COLOR_2, COLOR_1, COLOR_1],
                right_fac: 1.0,
            }),
            double_left: generate_gradient_theme!(BorderGradients {
                top: vec![COLOR_2, COLOR_1, COLOR_1],
                top_fac: 1.2,
                left: vec![COLOR_2, COLOR_1],
                left_fac: 1.2,
                bottom: vec![
                    COLOR_1,
                    COLOR_1,
                    get_transformed_rgb!(COLOR_2, 0.9, 1),
                ],
                bottom_fac: 1.6,
                right: vec![
                    COLOR_1,
                    get_transformed_rgb!(COLOR_2, 0.9, 1)
                ],
                right_fac: 0.8,
            }),
            top_left: generate_gradient_theme!(BorderGradients {
                top: vec![COLOR_2, COLOR_1],
                top_fac: 1.0,
                left: vec![COLOR_2, COLOR_1],
                left_fac: 1.0,
                bottom: vec![COLOR_1, COLOR_1],
                bottom_fac: 1.0,
                right: vec![COLOR_1, COLOR_1],
                right_fac: 1.0,
            }),
            top_right: generate_gradient_theme!(BorderGradients {
                top: vec![COLOR_1, COLOR_2],
                top_fac: 1.0,
                left: vec![COLOR_1, COLOR_1],
                left_fac: 1.0,
                bottom: vec![COLOR_1, COLOR_1],
                bottom_fac: 1.0,
                right: vec![COLOR_2, COLOR_1],
                right_fac: 1.0,
            }),
            double_horizontal: generate_gradient_theme!(
                BorderGradients {
                    top: vec![
                        COLOR_1,
                        get_transformed_rgb!(COLOR_2, 0.95, 2),
                        COLOR_1,
                    ],
                    top_fac: 1.0,
                    left: vec![
                        get_transformed_rgb!(COLOR_1, 1.1, 1),
                        get_transformed_rgb!(COLOR_1, 1.1, 1),
                    ],
                    left_fac: 1.0,
                    bottom: vec![
                        COLOR_1,
                        get_transformed_rgb!(COLOR_2, 0.95, 2),
                        COLOR_1
                    ],
                    bottom_fac: 1.0,
                    right: vec![
                        get_transformed_rgb!(COLOR_1, 1.1, 1),
                        get_transformed_rgb!(COLOR_1, 1.1, 1),
                    ],
                    right_fac: 1.0,
                }
            ),
            double_vertical: generate_gradient_theme!(
                BorderGradients {
                    top: vec![COLOR_1, COLOR_1],
                    top_fac: 1.0,
                    left: vec![
                        COLOR_1,
                        get_transformed_rgb!(COLOR_2, 0.9, 1),
                        COLOR_1
                    ],
                    left_fac: 1.0,
                    bottom: vec![COLOR_1, COLOR_1],
                    bottom_fac: 1.0,
                    right: vec![
                        COLOR_1,
                        get_transformed_rgb!(COLOR_2, 0.9, 1),
                        COLOR_1
                    ],
                    right_fac: 1.0,
                }
            ),
            up: generate_gradient_theme!(BorderGradients {
                top: vec![COLOR_2, COLOR_2],
                top_fac: 1.0,
                left: vec![COLOR_2, COLOR_1,],
                left_fac: 1.0,
                bottom: vec![COLOR_1, COLOR_1],
                bottom_fac: 1.0,
                right: vec![COLOR_2, COLOR_1],
                right_fac: 1.0,
            }),
            down: generate_gradient_theme!(BorderGradients {
                top: vec![COLOR_1, COLOR_1],
                top_fac: 1.0,
                left: vec![COLOR_1, COLOR_2,],
                left_fac: 1.0,
                bottom: vec![COLOR_2, COLOR_2],
                bottom_fac: 1.0,
                right: vec![COLOR_1, COLOR_2],
                right_fac: 1.0,
            }),
            left: generate_gradient_theme!(BorderGradients {
                top: vec![COLOR_2, COLOR_1,],
                top_fac: 1.0,
                left: vec![COLOR_2, COLOR_2,],
                left_fac: 1.0,
                bottom: vec![COLOR_2, COLOR_1,],
                bottom_fac: 1.0,
                right: vec![COLOR_1, COLOR_1,],
                right_fac: 1.0,
            }),

            right: generate_gradient_theme!(BorderGradients {
                top: vec![COLOR_1, COLOR_2,],
                top_fac: 1.0,
                left: vec![COLOR_1, COLOR_1,],
                left_fac: 1.0,
                bottom: vec![COLOR_1, COLOR_2,],
                bottom_fac: 1.0,
                right: vec![COLOR_2, COLOR_2,],
                right_fac: 1.0,
            }),
            base1: generate_gradient_theme!(BorderGradients {
                top: vec![
                    COLOR_3, COLOR_2, COLOR_2, COLOR_2, COLOR_3
                ],
                top_fac: 1.0,
                left: vec![
                    COLOR_3, COLOR_2, COLOR_2, COLOR_3, COLOR_2
                ],
                left_fac: 1.0,
                bottom: vec![COLOR_2, COLOR_3, COLOR_2],
                bottom_fac: 1.0,
                right: vec![COLOR_3, COLOR_2, COLOR_2],
                right_fac: 1.0,
            }),
            base2: generate_gradient_theme!(BorderGradients {
                top: vec![COLOR_3, COLOR_2, COLOR_2, COLOR_2],
                top_fac: 1.0,
                left: vec![COLOR_3, COLOR_2],
                left_fac: 1.0,
                bottom: vec![COLOR_2, COLOR_2, COLOR_3],
                bottom_fac: 1.0,
                right: vec![
                    COLOR_2,
                    (138, 73, 252),
                    COLOR_2,
                    COLOR_3
                ],
                right_fac: 1.0,
            }),
        };
}
