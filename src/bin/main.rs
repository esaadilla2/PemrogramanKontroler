#![no_std]
#![no_main]

use esp_backtrace as _;
use esp_println::println;
use nb::block;

use esp_hal::{
    analog::adc::{Adc, AdcConfig, Attenuation},
    delay::Delay,
    gpio::{Level, Output, OutputConfig},
    main,
    time::{Duration, Instant},
};

esp_bootloader_esp_idf::esp_app_desc!();

// =====================================
// KONSTANTA SENSOR
// =====================================

const VREF: f32 = 3.3;
const VS: f32 = 5.0;

// Threshold pressure hasil kalibrasi
const PRESSURE_THRESHOLD: f32 = 5.0;

#[main]
fn main() -> ! {

    // =====================================
    // INIT ESP32-S3
    // =====================================

    let peripherals =
        esp_hal::init(
            esp_hal::Config::default(),
        );

    let delay = Delay::new();

    // =====================================
    // LED GPIO2
    // =====================================

    let mut led = Output::new(
        peripherals.GPIO2,
        Level::Low,
        OutputConfig::default(),
    );

    // =====================================
    // BUZZER GPIO5
    // =====================================

    let mut buzzer = Output::new(
        peripherals.GPIO5,
        Level::Low,
        OutputConfig::default(),
    );

    // =====================================
    // ADC CONFIGURATION
    // =====================================

    let mut adc_config = AdcConfig::new();

    let mut pressure_pin =
        adc_config.enable_pin(
            peripherals.GPIO4,
            Attenuation::_11dB,
        );

    let mut adc =
        Adc::new(
            peripherals.ADC1,
            adc_config,
        );

    // =====================================
    // TASK INTERVAL
    // =====================================

    let sampling_interval =
        Duration::from_millis(100);

    let display_interval =
        Duration::from_millis(1000);

    // =====================================
    // TIMER
    // =====================================

    let mut previous_sample =
        Instant::now();

    let mut previous_display =
        Instant::now();

    // =====================================
    // VARIABLE
    // =====================================

    let mut pressure_raw: u16 = 0;
    let mut pressure: f32 = 0.0;

    // Counter jumlah anomaly berturut-turut
    let mut anomaly_counter: u32 = 0;

    let mut time_sec: u32 = 0;

    // =====================================
    // HEADER
    // =====================================

    println!();
    println!("====================================");
    println!("   PRESSURE MONITORING SYSTEM");
    println!("        ESP32-S3 + RUST");
    println!("Threshold : {:.2} kg/cm2", PRESSURE_THRESHOLD);
    println!("====================================");

    loop {

        let now = Instant::now();

        // =====================================
        // TASK 1 : ADC SAMPLING
        // =====================================

        if now - previous_sample >= sampling_interval {

            previous_sample = now;

            pressure_raw =
                block!(
                    adc.read_oneshot(
                        &mut pressure_pin,
                    )
                )
                .unwrap();

            // ADC -> Tegangan
            let vin =
                (pressure_raw as f32 / 4095.0)
                * VREF;

            // Tegangan -> Pressure
            pressure =
                ((vin / VS) - 0.04)
                / 0.09;

            // =====================================
            // ANOMALY DETECTION
            // =====================================

            if pressure < PRESSURE_THRESHOLD {

                led.set_high();
                buzzer.set_high();

                anomaly_counter += 1;

            } else {

                led.set_low();
                buzzer.set_low();

                anomaly_counter = 0;

            }

        }

        // =====================================
        // TASK 2 : SERIAL MONITOR
        // =====================================

        if now - previous_display >= display_interval {

            previous_display = now;

            time_sec += 1;

            println!();

            println!(
                "Time         : {} s",
                time_sec
            );

            println!(
                "ADC Raw      : {}",
                pressure_raw
            );

            println!(
                "Pressure     : {:.2} kg/cm2",
                pressure
            );

            println!(
                "Threshold    : {:.2} kg/cm2",
                PRESSURE_THRESHOLD
            );

            println!(
                "Anomaly Cnt  : {}",
                anomaly_counter
            );

            println!(
                "Status       : {}",
                if pressure < PRESSURE_THRESHOLD {
                    "ANOMALY"
                } else {
                    "NORMAL"
                }
            );

            println!(
                "LED          : {}",
                if pressure < PRESSURE_THRESHOLD {
                    "ON"
                } else {
                    "OFF"
                }
            );

            println!(
                "Buzzer       : {}",
                if pressure < PRESSURE_THRESHOLD {
                    "ON"
                } else {
                    "OFF"
                }
            );

        }

        // =====================================
        // LOOP DELAY
        // =====================================

        delay.delay_millis(10);

    }
}