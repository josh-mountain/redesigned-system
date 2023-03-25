use std::{thread, time};
use reqwest::blocking::get;
use serde_json::Value;
use epd_waveshare::{epd2in13_v3::EPD2in13, color::Black, prelude::*};
use rppal::spi::{Spi, Bus, Mode, SlaveSelect, Error};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Set up SPI
    let spi = Spi::new(Bus::Spi0, SlaveSelect::Ss0, 10_000_000, Mode::Mode0)?;
    
    // Set up pins for the display
    let cs_pin = rppal::gpio::Gpio::new()?.get(8)?.into_output();
    let busy_pin = rppal::gpio::Gpio::new()?.get(24)?.into_input();
    let dc_pin = rppal::gpio::Gpio::new()?.get(25)?.into_output();
    let rst_pin = rppal::gpio::Gpio::new()?.get(17)?.into_output();

    // create a new instance of the display
    let mut epd = EPD2in13::new(&mut spi, cs_pin, busy_pin, dc_pin, rst_pin)?;

    loop {
        // call the Kraken API to get the current prices for BTC in GBP
        let response = get("https://api.kraken.com/0/public/Ticker?pair=XBTGBP")?;
        let text = response.text()?;
        let json: Value = serde_json::from_str(&text)?;

        // extract the price from the JSON response
        let btc_price = json["result"]["XXBTZGBP"]["c"][0].as_str().unwrap();

        // clear the display and write the price
        epd.clear_frame(&mut spi)?;
        let _ = epd.set_text_color(Black);
        let _ = epd.set_background_color(White);
        let _ = epd.set_font(&sans_font);
        let _ = epd.draw_string_at(0, 0, &format!("BTC: £{}", btc_price));
        let _ = epd.update_frame(&mut spi)?;

        // sleep for 30 seconds before updating the price again
        thread::sleep(time::Duration::from_secs(30));
    }
}
