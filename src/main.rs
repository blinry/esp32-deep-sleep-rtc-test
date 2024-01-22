#![no_std]
#![no_main]

use core::time::Duration;
use esp32_hal::{
    clock::ClockControl, peripherals::Peripherals, prelude::*, rtc_cntl::sleep::TimerWakeupSource,
    Delay, Rtc,
};
use esp_backtrace as _;
use esp_println::println;

#[ram(rtc_fast)]
static mut RTC_FAST_DATA: u32 = 42;

#[ram(rtc_fast, uninitialized)]
static mut RTC_FAST_NOINIT: u32 = 42;

#[ram(rtc_fast, zeroed)]
static mut RTC_FAST_BSS: u32 = 42;

#[entry]
fn main() -> ! {
    let peripherals = Peripherals::take();
    let system = peripherals.SYSTEM.split();

    let clocks = ClockControl::max(system.clock_control).freeze();
    let mut delay = Delay::new(&clocks);
    let mut rtc = Rtc::new(peripherals.LPWR);

    unsafe {
        println!("Value of RTC_FAST_DATA: {}", RTC_FAST_DATA);
        println!("Value of RTC_FAST_NOINIT: {}", RTC_FAST_NOINIT);
        println!("Value of RTC_FAST_BSS: {}", RTC_FAST_BSS);

        RTC_FAST_DATA += 1;
        RTC_FAST_NOINIT += 1;
        RTC_FAST_BSS += 1;
    }

    println!("sleeping!");
    delay.delay_ms(100u32);

    let timer = TimerWakeupSource::new(Duration::from_secs(3));
    rtc.sleep_deep(&[&timer], &mut delay);
}
