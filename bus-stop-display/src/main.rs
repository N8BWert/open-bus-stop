//! This example test the RP Pico on board LED.
//!
//! It does not work with the RP Pico W board. See wifi_blinky.rs.

#![no_std]
#![no_main]

use embassy_executor::Spawner;
use embassy_rp::i2c::{Config, I2c};
use embassy_time::{Delay, Duration, Timer};
use {defmt_rtt as _, panic_probe as _};

use lcd1602_driver::{
    command::{DataWidth, State},
    lcd::{self, Basic, Ext, Lcd},
    sender::I2cSender,
};

fn two_digit_str(n: u8, buf: &mut [u8; 2]) -> &str {
    buf[0] = b'0' + (n / 10);
    buf[1] = b'0' + (n % 10);
    core::str::from_utf8(buf).unwrap()
}

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    let p = embassy_rp::init(Default::default());

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

    let mut time = 60;
    lcd.set_backlight(State::On);
    loop {
        for c in "Next Bus:  ".chars() {
            lcd.write_char_to_cur(c);
        }
        for c in "  101".chars() {
            lcd.write_char_to_cur(c);
        }

        lcd.set_cursor_pos((0, 1));
        if time > 0 {
            let mut buff = [0u8; 2];
            for c in two_digit_str(time, &mut buff).chars() {
                lcd.write_char_to_cur(c);
            }
            for c in "  min".chars() {
                lcd.write_char_to_cur(c);
            }
            time -= 1;
        } else {
            for c in "NOW".chars() {
                lcd.write_char_to_cur(c);
            }
            time = 60;
        }

        Timer::after(Duration::from_secs(5)).await;
        lcd.clean_display();
        lcd.return_home();
    }
}
