use cherry::widget::{
    container::{Alignment, Axis, Border, Container, Insets, Justification},
    text::Text,
    LayoutOptions, Widget,
};
use embedded_graphics::{
    draw_target::DrawTarget,
    mono_font::{ascii::FONT_10X20, MonoTextStyle},
    pixelcolor::Rgb888,
    prelude::{Point, RgbColor, Size},
    primitives::CornerRadii,
};
use embedded_graphics_simulator::{OutputSettingsBuilder, SimulatorDisplay, Window};
use std::convert::Infallible;

fn main() -> Result<(), Infallible> {
    let display_size = Size::new(300, 300);
    let mut display: SimulatorDisplay<Rgb888> = SimulatorDisplay::new(display_size);

    let character_style = MonoTextStyle::new(&FONT_10X20, Rgb888::BLACK);
    let text1 = Text::new("Line 1", character_style);
    let text2 = Text::new("Line 2", character_style);

    let rgb_swatch = Container::new()
        .alignment(Alignment::Center)
        .axis(Axis::Horizontal)
        .border(Some(Border {
            color: Rgb888::BLACK,
            width: 1,
        }))
        .corner_radii(Some(CornerRadii::new(Size::new(10, 10))))
        .justification(Justification::SpaceBetween)
        .padding(Insets::all(10))
        .width(Some(200))
        .children(vec![
            colored_container(Rgb888::RED, 25).boxed(),
            colored_container(Rgb888::GREEN, 40).boxed(),
            colored_container(Rgb888::BLUE, 55).boxed(),
        ]);

    let yellow_box = Container::new()
        .background_color(Some(Rgb888::YELLOW))
        .layout_options(
            LayoutOptions::new()
                .alignment(Some(Alignment::Stretch))
                .grow(1),
        );

    let magenta_box = Container::new()
        .background_color(Some(Rgb888::MAGENTA))
        .layout_options(LayoutOptions::new().grow(2))
        .width(Some(100));

    let container = Container::new()
        .alignment(Alignment::Center)
        .background_color(Some(Rgb888::WHITE))
        .border(Some(Border {
            color: Rgb888::GREEN,
            width: 4,
        }))
        .corner_radii(Some(CornerRadii::new(Size::new(10, 10))))
        .justification(Justification::SpaceAround)
        .margin(Insets::all(20))
        .children(vec![
            text1.boxed(),
            text2.boxed(),
            rgb_swatch.boxed(),
            yellow_box.boxed(),
            magenta_box.boxed(),
        ]);
    container.draw(&mut display, Point::zero(), display_size)?;

    let output_settings = OutputSettingsBuilder::new().build();
    Window::new("Hello World", &output_settings).show_static(&display);

    Ok(())
}

fn colored_container<Display>(color: Display::Color, size: u32) -> Container<Display>
where
    Display: DrawTarget,
{
    Container::new()
        .background_color(Some(color))
        .width(Some(size))
        .height(Some(size))
}
