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
    let pin_spi_ss = peripherals.pins.gpio33;
    WatchyDisplay::init(
        spi2,
        pin_spi_sck.into(),
        pin_spi_miso.into(),
        pin_spi_mosi.into(),
        pin_spi_ss.into(),
    );

    log::info!("Hello, world!");
    Ok(())
}
