use crate::display::WatchyDisplay;
use esp_idf_svc::hal::peripherals::Peripherals;
use esp_idf_svc::log::EspLogger;
use esp_idf_svc::sys;

mod display;

fn main() -> anyhow::Result<()> {
    sys::link_patches();
    EspLogger::initialize_default();

    let peripherals = Peripherals::take()?;
    let spi2 = peripherals.spi2;
    let pin_spi_sck = peripherals.pins.gpio47;
    let pin_spi_miso = peripherals.pins.gpio46;
    let pin_spi_mosi = peripherals.pins.gpio48;
    let pin_spi_edp_ss = peripherals.pins.gpio33;
    let pin_edp_dc = peripherals.pins.gpio34;
    let pin_edp_reset = peripherals.pins.gpio35;
    let pin_edp_busy = peripherals.pins.gpio36;
    let mut display = WatchyDisplay::new(
        spi2,
        pin_spi_sck.into(),
        pin_spi_miso.into(),
        pin_spi_mosi.into(),
        pin_spi_edp_ss.into(),
        pin_edp_dc.into(),
        pin_edp_reset.into(),
        pin_edp_busy.into(),
    )
    .expect("Failed to initialize display");
    display.draw_test().expect("Failed to draw test");

    log::info!("Hello, world!");
    Ok(())
}
