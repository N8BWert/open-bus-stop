//!
//! This is the main program for the informal bus stop display.
//!
//! It loads staticly created data for the bus stop and uses it to display the next bus
//! time on an LCD1602 display. It also uses the RTC to keep track of time and schedule alarms
//! to wake up the display when the next bus time is approaching.
//!

#![no_std]
#![no_main]

use embassy_executor::Spawner;
use embassy_rp::bind_interrupts;
use embassy_rp::i2c::{Config, I2c};
use embassy_rp::rtc::{DateTimeFilter, Rtc};
use embassy_time::{Delay, Duration, Timer};
use {defmt_rtt as _, panic_probe as _};

use lcd1602_driver::{
    command::{DataWidth, State},
    lcd::{self, Basic, Ext, Lcd},
    sender::I2cSender,
};

use bus_stop_display::{get_next_stop_time, START};

bind_interrupts!(struct Irqs {
    RTC_IRQ => embassy_rp::rtc::InterruptHandler;
});

fn two_digit_str(n: u8, buf: &mut [u8; 2]) -> &str {
    buf[0] = b'0' + (n / 10);
    buf[1] = b'0' + (n % 10);
    core::str::from_utf8(buf).unwrap()
}

fn three_digit_str(n: u16, buf: &mut [u8; 3]) -> &str {
    buf[0] = b'0' + (n / 100) as u8;
    if buf[0] == b'0' {
        buf[0] = b' ';
    }
    buf[1] = b'0' + (n % 100 / 10) as u8;
    if buf[1] == b'0' && buf[0] == b' ' {
        buf[1] = b' ';
    }
    buf[2] = b'0' + (n % 10) as u8;
    core::str::from_utf8(buf).unwrap()
}

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    let p = embassy_rp::init(Default::default());

    let mut rtc = Rtc::new(p.RTC, Irqs);
    if !rtc.is_running() {
        rtc.set_datetime(START).unwrap();
    }

    let mut i2c_config = Config::default();
    i2c_config.frequency = 50_000;
    i2c_config.sda_pullup = true;
    i2c_config.scl_pullup = true;
    let mut i2c = I2c::new_blocking(p.I2C0, p.PIN_5, p.PIN_4, i2c_config);
    let mut sender = I2cSender::new(&mut i2c, 0x27u8);
    let lcd_config = lcd::Config::default().set_data_width(DataWidth::Bit4);
    let mut delay = Delay;
    let mut lcd = Lcd::new(&mut sender, &mut delay, lcd_config, None);
    lcd.clean_display();
    lcd.return_home();

    lcd.set_backlight(State::On);
    loop {
        let mut now = rtc.now().unwrap();
        let next_stop_time = get_next_stop_time(&now);
        let mut minutes = next_stop_time.minutes_until_stop(now.hour, now.minute, now.second);
        defmt::info!("Minutes until next bus: {}", minutes);
        if minutes > 60 {
            lcd.set_backlight(State::Off);
            rtc.schedule_alarm(
                DateTimeFilter::default()
                    .hour((minutes / 60) as u8)
                    .minute((minutes % 60) as u8),
            );
            rtc.wait_for_alarm().await;
            rtc.disable_alarm();
            rtc.clear_interrupt();
            lcd.set_backlight(State::On);
            now = rtc.now().unwrap();
            minutes = next_stop_time.minutes_until_stop(now.hour, now.minute, now.second);
        }

        while !next_stop_time.passed(now.hour, now.minute, now.second) {
            minutes = next_stop_time.minutes_until_stop(now.hour, now.minute, now.second);

            // Display the next bus time
            for c in "Next Bus: ".chars() {
                lcd.write_char_to_cur(c);
            }
            for c in "  ".chars() {
                lcd.write_char_to_cur(c);
            }
            for c in three_digit_str(next_stop_time.route.get(), &mut [0u8; 3]).chars() {
                lcd.write_char_to_cur(c);
            }

            lcd.set_cursor_pos((0, 1));
            if minutes > 0 {
                for c in two_digit_str(minutes as u8, &mut [0u8; 2]).chars() {
                    lcd.write_char_to_cur(c);
                }
                for c in "  min".chars() {
                    lcd.write_char_to_cur(c);
                }

                Timer::after(Duration::from_secs(30)).await;
                now = rtc.now().unwrap();
            } else {
                for c in "NOW".chars() {
                    lcd.write_char_to_cur(c);
                }

                Timer::after(Duration::from_secs(30)).await;
                now = rtc.now().unwrap();
            }

            lcd.clean_display();
            lcd.return_home();
        }
    }
}
