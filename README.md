
# tui-gradient-block

### an extension to ratatui's block widget using colorgrad
### Note:
 - Complex gradients may have a brief delay in rendering (no more than 100 ms)
 - Heavily relies on my other widget called tui-rule (https://crates.io/crates/tui-rule)
 - A single block is 4 instances of the rule widget (still renders with minimal delay)
 - Code before 0.1.3 will be COMPLETELY outdated
 - Updating is highly recommended


### Features

- [x] customizable gradients that can be applied to
  - [x] fill text
  - [x] borders
  - [x] titles
- [x] fully customizable borders
- [x] fill text
- [x] padding
- [x] margin
- [x] pre-defined border styles
- [x] gradient themes 
    - 14 variations for each theme
```rust
fn render_gradient_block(frame: &mut Frame) {
    let block = GradientBlock::new()
	    .left_gradient(solid((48, 174, 209)))
        .bottom_gradient(solid((48, 174, 209)))
        .top_gradient(Box::new(
            GradientBuilder::new()
                .colors(&[
                    Color::from_rgba8(48, 174, 209, 1),
                    Color::from_rgba8(225, 22, 247, 1),
                ])
                .build::<colorgrad::LinearGradient>()
                .unwrap(),
        ))
        .right_gradient(Box::new(
            GradientBuilder::new()
                .colors(&[
                    Color::from_rgba8(225, 22, 247, 1),
                    Color::from_rgba8(48, 174, 209, 1),
                ])
                .build::<colorgrad::LinearGradient>()
                .unwrap(),
        ))
        .title_top(Line::from(generate_gradient_text!(
	            "Top title",
	            GradientBuilder::new().colors(&[
		            Color::from_rgba8(14, 67, 240), 
		            Color::from_rgba8(90, 34, 128)
			    ]).build::<colorgrad::LinearGradient>()
			    .unwrap()
        )).centered());

    frame.render_widget(block, frame.area());
}

```
![](https://iili.io/3dxrRcX.png)
## with multiple titles and border gradients
[![3dEyuvj.gif](https://iili.io/3dEyuvj.gif)](https://freeimage.host/)

## multiple pre-defined misc types
[![3dhR92I.md.png](https://iili.io/3dhR92I.md.png)](https://freeimage.host/i/3dhR92I)

## pre-defined gradient themes
![](https://iili.io/37y0pSt.png)
![](https://iili.io/37yE3kF.md.png)
Some of the gradients are from [colormagic](https://colormagic.app/)
