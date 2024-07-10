use anyhow::Result;
use embedded_graphics::geometry::Point;
use embedded_graphics::prelude::Primitive;
use embedded_graphics::primitives::{Circle, Line, PrimitiveStyle, PrimitiveStyleBuilder};
use embedded_graphics::Drawable;
use epd_waveshare::color::Color;
use epd_waveshare::epd1in54::{Display1in54, Epd1in54};
use epd_waveshare::prelude::WaveshareDisplay;
use esp_idf_svc::hal::delay::Delay;
use esp_idf_svc::hal::gpio::{AnyInputPin, AnyOutputPin, Input, Output, PinDriver};
use esp_idf_svc::hal::spi::config::{Config, DriverConfig};
use esp_idf_svc::hal::spi::{SpiDeviceDriver, SpiDriver, SPI2};
use esp_idf_svc::hal::units::FromValueType;

pub struct WatchyDisplay<'a> {
    epd: Epd1in54<
        SpiDeviceDriver<'a, SpiDriver<'a>>,
        PinDriver<'a, AnyInputPin, Input>,
        PinDriver<'a, AnyOutputPin, Output>,
        PinDriver<'a, AnyOutputPin, Output>,
        Delay,
    >,
    spi: SpiDeviceDriver<'a, SpiDriver<'a>>,
    delay: Delay,
}

impl<'a> WatchyDisplay<'a> {
    pub fn new(
        spi2: SPI2,
        pin_spi_sck: AnyOutputPin,
        pin_spi_miso: AnyInputPin,
        pin_spi_mosi: AnyOutputPin,
        pin_spi_edp_ss: AnyOutputPin,
        pin_edp_dc: AnyOutputPin,
        pin_edp_reset: AnyOutputPin,
        pin_edp_busy: AnyInputPin,
    ) -> Result<WatchyDisplay<'a>> {
        let mut spi = SpiDeviceDriver::new_single(
            spi2,
            pin_spi_sck,
            pin_spi_mosi,
            Some(pin_spi_miso),
            Some(pin_spi_edp_ss),
            &DriverConfig::new(),
            &Config::new().baudrate(20.MHz().into()),
        )
        .unwrap();
        let mut delay = Delay::new_default();
        let epd = Epd1in54::new(
            &mut spi,
            PinDriver::input(pin_edp_busy)?,
            PinDriver::output(pin_edp_dc)?,
            PinDriver::output(pin_edp_reset)?,
            &mut delay,
            None,
        )
        .unwrap();

        Ok(WatchyDisplay { epd, spi, delay })
    }

    pub fn draw_test(&mut self) -> Result<()> {
        // Use display graphics from embedded-graphics
        let mut display = Display1in54::default();

        // Use embedded graphics for drawing a line
        let style = PrimitiveStyleBuilder::new()
            .stroke_color(Color::Black)
            .stroke_width(1)
            .build();
        let _ = Line::new(Point::new(0, 120), Point::new(0, 295))
            .into_styled(style)
            .draw(&mut display);
        let _ = Circle::with_center(Point::new(50, 50), 50)
            .into_styled(PrimitiveStyle::with_fill(Color::White))
            .draw(&mut display);

        // Display updated frame
        self.epd
            .update_frame(&mut self.spi, &display.buffer(), &mut self.delay)?;
        self.epd.display_frame(&mut self.spi, &mut self.delay)?;

        // Set the EPD to sleep
        self.epd.sleep(&mut self.spi, &mut self.delay)?;
        Ok(())
    }
}
