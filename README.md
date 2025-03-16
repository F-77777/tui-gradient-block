# tui-gradient-block

### an extension to [ratatui](ratatui.rs)'s block widget with support for border gradients and a lot of different things that the ordinary block doesn't have, allowing for very visually appealing tuis.
# Note:

this crate is in the experimental versions and some things may not work as expected
complex border gradients may have a brief delay in rendering (no more than 300 ms)

### Features

- [x] customizable gradients that can be applied to
  - [x] fill text
  - [x] borders
  - [x] titles
- [x] fully customizable borders
- [x] fill text
- [x] pre-defined border styles
- [x] border themes (midnight blurple, misty blue, rusty ruins)
```rust
fn render_gradient_block(frame: &mut Frame) {
    let gradblock = tui_gradientblock::new(&frame.area(), SplitBorderSegments::NONE)
    // Using Vec<GradientSegment, Gradient> works, but it may be tedious and make code unreadable. we (im the only one in this project) recommend using the generate_gradient_theme macro for simplicity
        .set_gradients(generate_gradient_theme!(
            BorderGradients {
                left: vec![(48, 174, 209), (48, 174, 209)],
                bottom: vec![(48, 174, 209), (48, 174, 209)],
                right: vec![(225, 22, 247), (48, 174, 209)],
                top: vec![(48, 174, 209), (225, 22, 247)],
                left_fac: 1.0,
                bottom_fac: 1.0,
                right_fac: 1.0,
                top_fac: 1.0,
            }
        ))
        .top_titles(vec![(
            "Top title".to_owned(),
            TitleAlignment::Centered,
            Some((vec![(14, 67, 240), (90, 34, 128)], 1.0)),
        )])
        .set_lines();

    frame.render_widget(gradblock, frame.area());
}

```
![](https://iili.io/3dxrRcX.png)
## with multiple titles and border gradients
[![3dEyuvj.gif](https://iili.io/3dEyuvj.gif)](https://freeimage.host/)

## multiple pre-defined misc types
[![3dhR92I.md.png](https://iili.io/3dhR92I.md.png)](https://freeimage.host/i/3dhR92I)

## pre-defined gradient themes
the shown gradient is rusty ruins, and each gradient theme has 14 (soon to be 16) variations
![rusty ruins](https://iili.io/33DEVjI.gif)

please give me recommendations on pre-defined border themes and border styles