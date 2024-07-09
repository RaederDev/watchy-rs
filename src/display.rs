// #define EPD_SPI_SCK 47
// #define EPD_SPI_MISO 46
// #define EPD_SPI_MOSI 48
// #define EPD_SPI_SS 33

use esp_idf_svc::hal::delay::Delay;
use esp_idf_svc::hal::gpio::{AnyInputPin, AnyOutputPin};
use esp_idf_svc::hal::spi::config::{Config, DriverConfig};
use esp_idf_svc::hal::spi::{SpiDeviceDriver, SPI2};
use esp_idf_svc::hal::units::FromValueType;

pub struct WatchyDisplay;

impl WatchyDisplay {
    pub fn init(
        spi2: SPI2,
        pin_sck: AnyOutputPin,
        pin_miso: AnyInputPin,
        pin_mosi: AnyOutputPin,
        pin_ss: AnyOutputPin,
    ) {
        let mut spi = SpiDeviceDriver::new_single(
            spi2,
            pin_sck,
            pin_mosi,
            Some(pin_miso),
            Some(pin_ss),
            &DriverConfig::new(),
            &Config::new().baudrate(20.MHz().into()),
        );
        let mut delay = Delay::new_default();
        //
        // let mut spi = Spi::new(spi2, 20.MHz(), SpiMode::Mode0, clocks).with_pins(
        //     Some(pin_spi_sck),
        //     Some(pin_spi_mosi),
        //     Some(pin_spi_miso),
        //     Some(pin_spi_ss),
        // );
        // let mut delay = Delay;
        // let mut epd = Epd1in54::new(&mut spi, OutputPin, busy_in, dc, rst, &mut delay);
    }
}

fn setup_display() {
    // Setup EPD
    // let mut epd = Epd1in54::new(&mut spi, cs_pin, busy_in, dc, rst, &mut delay)?;
    //
    // // Use display graphics from embedded-graphics
    // let mut display = Display1in54::default();
    //
    // // Use embedded graphics for drawing a line
    // let style = PrimitiveStyleBuilder::new()
    //     .stroke_color(Black)
    //     .stroke_width(1)
    //     .build();
    // let _ = Line::new(Point::new(0, 120), Point::new(0, 295))
    //     .into_styled(style)
    //     .draw(&mut display);
    //
    // // Display updated frame
    // epd.update_frame(&mut spi, &display.buffer(), &mut delay)?;
    // epd.display_frame(&mut spi, &mut delay)?;
    //
    // // Set the EPD to sleep
    // epd.sleep(&mut spi, &mut delay)?;
}
