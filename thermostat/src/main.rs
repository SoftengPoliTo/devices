//! HTTP Server with REST API handler
//!
//! Go to 192.168.1.126 to test

//use std::sync::{Arc, Mutex};

//use ascot::hazards::Hazard;
//use ascot::route::Route;

//use ascot_esp32c3::device::{DeviceAction, ResponseBuilder};
// TODO: Add thermostat to ascot_esp32c3
//use ascot_esp32c3::devices::thermostat::Thermostat;
//use ascot_esp32c3::server::Server;
//use ascot_esp32c3::service::ServiceConfig;
use ascot_esp32c3::wifi::Wifi;

//use esp_idf_svc::hal::delay::Ets;
//use esp_idf_svc::hal::gpio::PinDriver;
use esp_idf_svc::hal::prelude::Peripherals;
use esp_idf_svc::log::EspLogger;

#[toml_cfg::toml_config]
pub struct DeviceConfig {
    #[default("")]
    ssid: &'static str,
    #[default("")]
    password: &'static str,
    #[default("ascot")]
    hostname: &'static str,
    #[default("ascot")]
    service: &'static str,
}

fn main() -> ascot_esp32c3::error::Result<()> {
    // A hack to make sure that a few patches to the ESP-IDF which are
    // implemented in Rust are linked to the final executable
    esp_idf_svc::sys::link_patches();

    // Bind the log crate to the ESP Logging facilities
    EspLogger::initialize_default();

    // `async-io` uses the ESP IDF `eventfd` syscall to implement async IO.
    // If you use `tokio`, you still have to do the same as
    // `tokio` also uses the `eventfd` syscall
    let _event = esp_idf_svc::io::vfs::MountedEventfs::mount(5)?;

    // Retrieve all esp32c3 peripherals (singleton)
    let peripherals = Peripherals::take()?;

    // Retrieve device configuration
    let device_config = DEVICE_CONFIG;

    // Connects to Wi-Fi.
    let _wifi = Wifi::connect_wifi(
        device_config.ssid,
        device_config.password,
        peripherals.modem,
    )?;

    // TODO
    /*

    // TODO: Add thermostat pins for esp32-c3

    let temperature_action = DeviceAction::new(
        // Configuration for the `GET` temperature route.
        Route::get("/temperature").description("Retrieve air temperature."),
        // TODO: Implement a response for temperature
        ResponseBuilder(|req| req.into_ok_response(), "Temperature detected!"),
    )
    .body(move || {
        // TODO: Retrieve temperature

        Ok(())
    });

    let humidity_action = DeviceAction::new(
        // Configuration for the `GET` humidity route.
        Route::get("/off").description("Retrieve air humidity."),
        // TODO: Implement a response for humidity.
        ResponseBuilder(|req| req.into_ok_response(), "Humidity detected!"),
    )
    .body(move || {
        // TODO: Retrieve temperature

        Ok(())
    });

    // TODO: Implement thermostat
    //let thermostat = Thermostat::new(temperature_action, humidity_action).build();

    // TODO: Run server
    Server::new(thermostat)
    .service(
        ServiceConfig::mdns_sd(wifi.ip)
            .hostname(device_config.hostname)
            .domain_name(device_config.service),
    )
    .run()*/
    Ok(())
}
