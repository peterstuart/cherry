use cherry::widget::{
    container::{Border, Container, Options},
    text::{Options as TextOptions, Text},
    Widget,
};
use embedded_graphics::{
    mono_font::{ascii::FONT_10X20, MonoTextStyle},
    pixelcolor::Rgb888,
    prelude::{Point, RgbColor, Size},
    primitives::CornerRadii,
};
use embedded_graphics_simulator::{OutputSettingsBuilder, SimulatorDisplay, Window};
use std::convert::Infallible;

fn main() -> Result<(), Infallible> {
    let mut display: SimulatorDisplay<Rgb888> = SimulatorDisplay::new(Size::new(300, 300));

    let character_style = MonoTextStyle::new(&FONT_10X20, Rgb888::YELLOW);
    let text1 = Text::new(TextOptions { character_style }, "Line 1");
    let text2 = Text::new(TextOptions { character_style }, "Line 2");
    let text3 = Text::new(TextOptions { character_style }, "Line 3");

    let container = Container::new(
        Options {
            background_color: Some(Rgb888::RED),
            border: Some(Border {
                color: Rgb888::GREEN,
                width: 4,
            }),
            corner_radii: Some(CornerRadii::new(Size::new(10, 10))),
        },
        vec![text1.boxed(), text2.boxed(), text3.boxed()],
    );
    container.draw(&mut display, Point::new(20, 20), Size::new(260, 260))?;

    let output_settings = OutputSettingsBuilder::new().build();
    Window::new("Hello World", &output_settings).show_static(&display);

    Ok(())
}
