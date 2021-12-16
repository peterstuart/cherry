use cherry::widget::{
    container::{Border, Container, Options},
    Widget,
};
use embedded_graphics::{
    pixelcolor::Rgb888,
    prelude::{Point, RgbColor, Size},
    primitives::CornerRadii,
};
use embedded_graphics_simulator::{OutputSettingsBuilder, SimulatorDisplay, Window};
use std::convert::Infallible;

fn main() -> Result<(), Infallible> {
    let mut display: SimulatorDisplay<Rgb888> = SimulatorDisplay::new(Size::new(300, 300));

    let widget = Container::new(Options {
        background_color: Some(Rgb888::RED),
        border: Some(Border {
            color: Rgb888::GREEN,
            width: 4,
        }),
        corner_radii: Some(CornerRadii::new(Size::new(10, 10))),
    });
    widget.draw(&mut display, Point::new(20, 20), Size::new(260, 260))?;

    let output_settings = OutputSettingsBuilder::new().build();
    Window::new("Hello World", &output_settings).show_static(&display);

    Ok(())
}
