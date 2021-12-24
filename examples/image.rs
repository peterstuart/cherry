use cherry::widget::{image::Image, Widget};
use embedded_graphics::{
    pixelcolor::Rgb888,
    prelude::{Point, Size},
};
use embedded_graphics_simulator::{OutputSettingsBuilder, SimulatorDisplay, Window};
use std::convert::Infallible;
use tinybmp::Bmp;

fn main() -> Result<(), Infallible> {
    let mut display: SimulatorDisplay<Rgb888> = SimulatorDisplay::new(Size::new(300, 300));

    let image_data = include_bytes!("./images/cherry.bmp");
    let bmp = Bmp::<Rgb888>::from_slice(image_data).unwrap();

    let widget = Image::new(&bmp);
    widget.draw(&mut display, Point::new(100, 100), Size::new(100, 100))?;

    let output_settings = OutputSettingsBuilder::new().build();
    Window::new("Hello World", &output_settings).show_static(&display);

    Ok(())
}
