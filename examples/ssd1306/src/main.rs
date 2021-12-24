#![no_main]
#![no_std]
#![feature(default_alloc_error_handler)]

extern crate alloc;

use alloc::vec;
use alloc_cortex_m::CortexMHeap;
use cherry::widget::{
    container::{Alignment, Axis, Border, Container, Insets, Justification},
    Widget,
};
use defmt_rtt as _;
use embedded_graphics::{pixelcolor::BinaryColor, prelude::*, primitives::CornerRadii};
use panic_probe as _;
use ssd1306::{mode::BufferedGraphicsMode, prelude::*, I2CDisplayInterface, Ssd1306};
use stm32f3_discovery::stm32f3xx_hal::{
    self as _,
    gpio::{Alternate, Gpiob, OpenDrain, Pin, U},
    i2c::I2c,
    pac::{self, I2C1},
    prelude::*,
};

type Screen = Ssd1306<
    I2CInterface<
        I2c<
            I2C1,
            (
                Pin<Gpiob, U<6_u8>, Alternate<OpenDrain, 4_u8>>,
                Pin<Gpiob, U<7_u8>, Alternate<OpenDrain, 4_u8>>,
            ),
        >,
    >,
    ssd1306::prelude::DisplaySize128x64,
    BufferedGraphicsMode<ssd1306::prelude::DisplaySize128x64>,
>;

const HEAP_SIZE: usize = 1024;

const MAX_BLOCK_SIZE: u32 = 30;
const MIN_BLOCK_SIZE: u32 = 0;
const BLOCK_SIZE_INCREMENT: u32 = 2;

#[global_allocator]
static ALLOCATOR: CortexMHeap = CortexMHeap::empty();

#[cortex_m_rt::entry]
fn main() -> ! {
    init_allocator();

    let mut display = configure_display();
    animate(&mut display);
}

fn init_allocator() {
    static mut HEAP: [u8; HEAP_SIZE] = [0; HEAP_SIZE];

    unsafe {
        ALLOCATOR.init(
            &mut HEAP as *const u8 as usize,
            core::mem::size_of_val(&HEAP),
        )
    }
}

fn configure_display() -> Screen {
    let peripherals = pac::Peripherals::take().unwrap();

    let mut flash = peripherals.FLASH.constrain();
    let mut rcc = peripherals.RCC.constrain();
    let clocks = rcc.cfgr.freeze(&mut flash.acr);

    let mut gpiob = peripherals.GPIOB.split(&mut rcc.ahb);

    let mut scl =
        gpiob
            .pb6
            .into_af4_open_drain(&mut gpiob.moder, &mut gpiob.otyper, &mut gpiob.afrl);
    let mut sda =
        gpiob
            .pb7
            .into_af4_open_drain(&mut gpiob.moder, &mut gpiob.otyper, &mut gpiob.afrl);
    scl.internal_pull_up(&mut gpiob.pupdr, true);
    sda.internal_pull_up(&mut gpiob.pupdr, true);

    let i2c = I2c::new(
        peripherals.I2C1,
        (scl, sda),
        400.kHz().try_into().unwrap(),
        clocks,
        &mut rcc.apb1,
    );

    let interface = I2CDisplayInterface::new(i2c);

    let mut display = Ssd1306::new(interface, DisplaySize128x64, DisplayRotation::Rotate0)
        .into_buffered_graphics_mode();
    display.init().unwrap();

    display
}

fn animate(display: &mut Screen) -> ! {
    let size = display.size();
    let mut ascending = true;
    let mut block_size: u32 = MIN_BLOCK_SIZE;

    loop {
        display.clear();

        let widget = widget(block_size);
        widget.draw(display, Point::zero(), size).unwrap();

        display.flush().unwrap();

        if ascending && block_size == MAX_BLOCK_SIZE {
            ascending = false;
        } else if !ascending && block_size == MIN_BLOCK_SIZE {
            ascending = true
        }

        if ascending {
            block_size += BLOCK_SIZE_INCREMENT;
        } else {
            block_size -= BLOCK_SIZE_INCREMENT;
        }
    }
}

fn widget<Display>(block_size: u32) -> Container<Display>
where
    Display: 'static + DrawTarget<Color = BinaryColor>,
{
    Container::new()
        .alignment(Alignment::Center)
        .axis(Axis::Horizontal)
        .border(Some(Border {
            color: BinaryColor::On,
            width: 1,
        }))
        .children(vec![
            block(block_size).boxed(),
            block(MAX_BLOCK_SIZE - block_size).boxed(),
            block(block_size).boxed(),
        ])
        .corner_radii(Some(CornerRadii::new(Size::new(10, 10))))
        .justification(Justification::SpaceBetween)
        .padding(Insets::all(10))
        .width(Some(200))
}

fn block<Display>(size: u32) -> Container<Display>
where
    Display: DrawTarget<Color = BinaryColor>,
{
    Container::new()
        .background_color(Some(BinaryColor::On))
        .width(Some(size))
        .height(Some(size))
}
