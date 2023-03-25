use std::{thread, time};
use reqwest::blocking::get;
use serde_json::Value;
use epd_waveshare::{epd2in13_v3::EPD2in13, color::Black, prelude::*};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // set up SPI pins and fonts
    let mut spi = spidev::Spidev::open("/dev/spidev0.0")?;
    let spi_options = spidev::SpidevOptions::new()
        .max_speed_hz(4_000_000)
        .mode(spidev::SpiModeFlags::SPI_MODE_0)
        .build();
    spi.configure(&spi_options)?;

    let cs_pin = sysfs_gpio::Pin::new(8);
    let busy_pin = sysfs_gpio::Pin::new(25);
    let dc_pin = sysfs_gpio::Pin::new(24);
    let rst_pin = sysfs_gpio::Pin::new(17);
    let mut delay = std::thread::sleep;

    let sans_font = include_bytes!("../fonts/DejaVuSans.ttf") as &[u8];

    // create a new instance of the display
    let mut epd = EPD2in13::new(&mut spi, cs_pin, busy_pin, dc_pin, rst_pin)?;

    loop {
        // call the Kraken API to get the current prices for BTC, XMR, and XTZ in GBP
        let response = get("https://api.kraken.com/0/public/Ticker?pair=XXBTZGBP,XMRGBP,XTZGBP")?;
        let text = response.text()?;
        let json: Value = serde_json::from_str(&text)?;

        // extract the prices from the JSON response
        let btc_price = json["result"]["XXBTZGBP"]["c"][0].as_str().unwrap();
        let xmr_price = json["result"]["XMRGBP"]["c"][0].as_str().unwrap();
        let xtz_price = json["result"]["XTZGBP"]["c"][0].as_str().unwrap();

        // clear the display and write the prices
        epd.clear_frame(&mut spi)?;
        let _ = epd.set_text_color(Black);
        let _ = epd.set_background_color(White);
        let _ = epd.set_font(&sans_font);
        let _ = epd.draw_string_at(0, 0, &format!("BTC: £{}", btc_price));
        let _ = epd.draw_string_at(0, 20, &format!("XMR: £{}", xmr_price));
        let _ = epd.draw_string_at(0, 40, &format!("XTZ: £{}", xtz_price));
        let _ = epd.update_frame(&mut spi)?;

        // sleep for 30 seconds before updating the prices again
        thread::sleep(time::Duration::from_secs(30));
    }
}
