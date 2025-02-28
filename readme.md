# tui-gradient-block

### an extension to [ratatui](ratatui.rs)'s block widget with support for border gradients and a lot of different things that the ordinary block doesn't have, allowing for very visually appealing tuis.

### Features

- [x] customizable gradients that can be applied to
  - [x] fill text
  - [x] borders
  - [x] titles
- [x] fully customizable borders
- [x] fill text
- [x] pre-defined border styles

### Planned features
- [ ] margin
- [ ] pre-defined gradient themes 
```rust
fn render_gradient_block() {
    let gradblock = tui_gradientblock::new(&frame.area(), SplitBorderSegments::NONE)
        .set_gradients(vec![
            (
                GradientSegments::Right,
                vec![(251, 8, 255), (10, 100, 112)],
                1.0,
            ),
            (
                GradientSegments::Top,
                vec![(10, 100, 112), (251, 8, 255)],
                1.0,
            ),
            (
                GradientSegments::Left,
                vec![(10, 100, 112), (10, 100, 112)],
                1.0,
            ),
            (
                GradientSegments::Bottom,
                vec![(10, 100, 112), (10, 100, 112)],
                1.0,
            ),
        ])
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


this crate was made by an intermediate frontend web developer and a beginner to rust who started learning about 4 months ago.

please give me recommendations on pre-defined border themes as your input would be very helpful.