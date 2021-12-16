use cherry::widget::{
    text::{Options, Text},
    Widget,
};
use embedded_graphics::{
    mono_font::{ascii::FONT_10X20, MonoTextStyle},
    pixelcolor::Rgb888,
    prelude::{Point, RgbColor, Size},
};
use embedded_graphics_simulator::{OutputSettingsBuilder, SimulatorDisplay, Window};
use std::convert::Infallible;

fn main() -> Result<(), Infallible> {
    let mut display: SimulatorDisplay<Rgb888> = SimulatorDisplay::new(Size::new(300, 300));

    let character_style = MonoTextStyle::new(&FONT_10X20, Rgb888::YELLOW);
    let widget = Text::new(Options { character_style });
    widget.draw(&mut display, Point::new(20, 20), Size::new(100, 100))?;

    let output_settings = OutputSettingsBuilder::new().build();
    Window::new("Hello World", &output_settings).show_static(&display);

    Ok(())
}
