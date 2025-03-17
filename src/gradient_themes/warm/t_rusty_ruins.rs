crate::generate_theme_use!();
const COLOR_1: (u8, u8, u8) = (134, 56, 44);
const COLOR_2: (u8, u8, u8) = (229, 110, 85);
const COLOR_3: (u8, u8, u8) = (246, 180, 100);
lazy_static! {
    pub static ref RUSTY_RUINS_TITLES: TitleSet =
        gen_titles!(COLOR_1, COLOR_2, COLOR_3);
    pub static ref RUSTY_RUINS: GradientThemeSet =
        GradientThemeSet {
            bottom_right: generate_gradient_theme!(
                BorderGradients {
                    top: vec![COLOR_1, COLOR_1],
                    top_fac: 1.0,
                    left: vec![COLOR_1, COLOR_1],
                    left_fac: 1.0,
                    bottom: vec![
                        COLOR_1, COLOR_2, COLOR_3
                    ],
                    bottom_fac: 1.0,
                    right: vec![
                        COLOR_1, COLOR_2, COLOR_3
                    ],
                    right_fac: 1.0,
                }
            ),
            bottom_left: generate_gradient_theme!(
                BorderGradients {
                    top: vec![COLOR_1, COLOR_1],
                    top_fac: 1.0,
                    left: vec![
                        COLOR_1, COLOR_2, COLOR_3
                    ],
                    left_fac: 1.0,
                    bottom: vec![
                        COLOR_3, COLOR_2, COLOR_1
                    ],
                    bottom_fac: 1.0,
                    right: vec![COLOR_1, COLOR_1],
                    right_fac: 1.0,
                }
            ),
            double_right: generate_gradient_theme!(
                BorderGradients {
                    top: vec![
                        COLOR_1, COLOR_1, COLOR_3
                    ],
                    top_fac: 1.0,
                    left: vec![
                        COLOR_1, COLOR_2, COLOR_3
                    ],
                    left_fac: 1.0,
                    bottom: vec![
                        COLOR_3, COLOR_2, COLOR_1
                    ],
                    bottom_fac: 1.0,
                    right: vec![
                        COLOR_3, COLOR_2, COLOR_1
                    ],
                    right_fac: 1.0,
                }
            ),
            double_left: generate_gradient_theme!(
                BorderGradients {
                    top: vec![
                        COLOR_3, COLOR_2, COLOR_1
                    ],
                    top_fac: 1.0,
                    left: vec![
                        COLOR_3, COLOR_2, COLOR_1
                    ],
                    left_fac: 1.0,
                    bottom: vec![
                        COLOR_1, COLOR_2, COLOR_3
                    ],
                    bottom_fac: 1.0,
                    right: vec![
                        COLOR_1, COLOR_2, COLOR_3
                    ],
                    right_fac: 1.0,
                }
            ),
            top_left: generate_gradient_theme!(
                BorderGradients {
                    top: vec![
                        COLOR_3, COLOR_2, COLOR_1
                    ],
                    top_fac: 1.0,
                    left: vec![
                        COLOR_3, COLOR_2, COLOR_1
                    ],
                    left_fac: 1.0,
                    bottom: vec![
                        COLOR_1, COLOR_1
                    ],
                    bottom_fac: 1.0,
                    right: vec![COLOR_1, COLOR_1],
                    right_fac: 1.0,
                }
            ),
            top_right: generate_gradient_theme!(
                BorderGradients {
                    top: vec![
                        COLOR_1, COLOR_2, COLOR_3
                    ],
                    top_fac: 1.0,
                    left: vec![COLOR_1, COLOR_1],
                    left_fac: 1.0,
                    bottom: vec![
                        COLOR_1, COLOR_1
                    ],
                    bottom_fac: 1.0,
                    right: vec![
                        COLOR_3, COLOR_2, COLOR_1
                    ],
                    right_fac: 1.0,
                }
            ),
            double_horizontal: generate_gradient_theme!(
                BorderGradients {
                    top: vec![
                        COLOR_1, COLOR_2,
                        COLOR_3, COLOR_2,
                        COLOR_1
                    ],
                    top_fac: 1.0,
                    left: vec![
                        get_transformed_rgb!(
                            COLOR_1, 1.1, 1
                        ),
                        get_transformed_rgb!(
                            COLOR_1, 1.1, 1
                        ),
                    ],
                    left_fac: 1.0,
                    bottom: vec![
                        COLOR_1, COLOR_2,
                        COLOR_3, COLOR_2,
                        COLOR_1
                    ],
                    bottom_fac: 1.0,
                    right: vec![
                        get_transformed_rgb!(
                            COLOR_1, 1.1, 1
                        ),
                        get_transformed_rgb!(
                            COLOR_1, 1.1, 1
                        ),
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
                        get_transformed_rgb!(
                            COLOR_2, 0.9, 1
                        ),
                        COLOR_3,
                        get_transformed_rgb!(
                            COLOR_2, 0.9, 2
                        ),
                        COLOR_1
                    ],
                    left_fac: 1.0,
                    bottom: vec![
                        COLOR_1, COLOR_1
                    ],
                    bottom_fac: 1.0,
                    right: vec![
                        COLOR_1,
                        get_transformed_rgb!(
                            COLOR_2, 0.9, 1
                        ),
                        COLOR_3,
                        get_transformed_rgb!(
                            COLOR_2, 0.9, 2
                        ),
                        COLOR_1
                    ],
                    right_fac: 1.0,
                }
            ),
            up: generate_gradient_theme!(
                BorderGradients {
                    top: vec![
                        get_transformed_rgb!(
                            COLOR_2, 1.1, 1
                        ),
                        get_transformed_rgb!(
                            COLOR_2, 1.1, 1
                        )
                    ],
                    top_fac: 1.0,
                    left: vec![
                        get_transformed_rgb!(
                            COLOR_2, 1.1, 1
                        ),
                        COLOR_2,
                        COLOR_1
                    ],
                    left_fac: 1.0,
                    bottom: vec![
                        COLOR_1, COLOR_1
                    ],
                    bottom_fac: 1.0,
                    right: vec![
                        get_transformed_rgb!(
                            COLOR_2, 1.1, 1
                        ),
                        COLOR_2,
                        COLOR_1
                    ],
                    right_fac: 1.0,
                }
            ),
            down: generate_gradient_theme!(
                BorderGradients {
                    top: vec![COLOR_1, COLOR_1],
                    top_fac: 1.0,
                    left: vec![COLOR_1, COLOR_2],
                    left_fac: 1.0,
                    bottom: vec![
                        COLOR_2, COLOR_2
                    ],
                    bottom_fac: 1.0,
                    right: vec![COLOR_1, COLOR_2],
                    right_fac: 1.0,
                }
            ),
            left: generate_gradient_theme!(
                BorderGradients {
                    top: vec![
                        COLOR_3, COLOR_2, COLOR_1
                    ],
                    top_fac: 1.0,
                    left: vec![COLOR_3, COLOR_3],
                    left_fac: 1.0,
                    bottom: vec![
                        COLOR_3, COLOR_2, COLOR_1
                    ],
                    bottom_fac: 1.0,
                    right: vec![COLOR_1, COLOR_1],
                    right_fac: 1.0,
                }
            ),

            right: generate_gradient_theme!(
                BorderGradients {
                    top: vec![
                        COLOR_1, COLOR_2, COLOR_3
                    ],
                    top_fac: 1.0,
                    left: vec![COLOR_1, COLOR_1],
                    left_fac: 1.0,
                    bottom: vec![
                        COLOR_1, COLOR_2, COLOR_3
                    ],
                    bottom_fac: 1.0,
                    right: vec![COLOR_3, COLOR_3],
                    right_fac: 1.0,
                }
            ),
            base1: generate_gradient_theme!(
                BorderGradients {
                    top: vec![
                        COLOR_1, COLOR_2,
                        COLOR_3, COLOR_2
                    ],
                    top_fac: 1.0,
                    left: vec![COLOR_1, COLOR_2],
                    left_fac: 1.0,
                    bottom: vec![
                        COLOR_2, COLOR_2, COLOR_3
                    ],
                    bottom_fac: 1.0,
                    right: vec![COLOR_2, COLOR_3],
                    right_fac: 1.0,
                }
            ),
            base2: generate_gradient_theme!(
                BorderGradients {
                    top: vec![COLOR_3, COLOR_2],
                    top_fac: 1.0,
                    left: vec![COLOR_3, COLOR_2],
                    left_fac: 1.0,
                    bottom: vec![
                        COLOR_2, COLOR_2, COLOR_2
                    ],
                    bottom_fac: 1.0,
                    right: vec![
                        COLOR_2, COLOR_1, COLOR_2
                    ],
                    right_fac: 1.0,
                }
            ),
        };
}
