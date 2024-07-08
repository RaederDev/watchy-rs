#![no_std]
#![no_main]

use embassy_executor::Spawner;
use embassy_time::{Delay, Duration, Timer};
use esp_backtrace as _;
use esp_hal::{
    clock::ClockControl,
    peripherals::Peripherals,
    prelude::*,
};
use esp_hal::system::SystemControl;
use esp_hal::timer::timg::TimerGroup;
// extern crate alloc;
// use core::mem::MaybeUninit;

// #[global_allocator]
// static ALLOCATOR: esp_alloc::EspHeap = esp_alloc::EspHeap::empty();
//
// fn init_heap() {
//     const HEAP_SIZE: usize = 32 * 1024;
//     static mut HEAP: MaybeUninit<[u8; HEAP_SIZE]> = MaybeUninit::uninit();
//
//     unsafe {
//         ALLOCATOR.init(HEAP.as_mut_ptr() as *mut u8, HEAP_SIZE);
//     }
// }

#[embassy_executor::task]
async fn run() {
    loop {
        log::info!("Hello from an embassy thread");
        Timer::after(Duration::from_millis(1_000)).await;
    }
}

#[main]
async fn main(spawner: Spawner) {
    let peripherals = Peripherals::take();
    let system = SystemControl::new(peripherals.SYSTEM);
    let clocks = ClockControl::max(system.clock_control).freeze();

    esp_println::logger::init_logger_from_env();

    let timg0 = TimerGroup::new_async(peripherals.TIMG0, &clocks);
    esp_hal_embassy::init(&clocks, timg0);

    spawner.spawn(run()).ok();

    // let timer = esp_hal::timer::timg::TimerGroup::new(peripherals.TIMG1, &clocks, None).timer0;
    // let _init = esp_wifi::initialize(
    //     esp_wifi::EspWifiInitFor::Wifi,
    //     timer,
    //     esp_hal::rng::Rng::new(peripherals.RNG),
    //     peripherals.RADIO_CLK,
    //     &clocks,
    // )
    // .unwrap();

    loop {
        log::info!("Hello world!");
        Timer::after(Duration::from_millis(2_000)).await;
    }
}
