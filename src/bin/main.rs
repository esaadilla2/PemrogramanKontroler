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

#[main]
fn main() -> ! {

    // =====================================
    // INIT ESP32-S3
    // =====================================

    let peripherals =
        esp_hal::init(
            esp_hal::Config::default()
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
    // POTENTIOMETER -> GPIO4
    // =====================================

    let mut adc_config = AdcConfig::new();

    let mut pressure_pin =
        adc_config.enable_pin(
            peripherals.GPIO4,
            Attenuation::_11dB,
        );

    let mut adc = Adc::new(
        peripherals.ADC1,
        adc_config,
    );

    // =====================================
    // REALTIME TASK INTERVAL
    // =====================================

    let sampling_interval =
        Duration::from_millis(100);

    let display_interval =
        Duration::from_millis(1000);

    let watchdog_interval =
        Duration::from_millis(3000);

    // =====================================
    // TASK TIMER
    // =====================================

    let mut previous_sample =
        Instant::now();

    let mut previous_display =
        Instant::now();

    let mut previous_watchdog =
        Instant::now();

    // =====================================
    // SYSTEM VARIABLE
    // =====================================

    let mut pressure_value: u16 = 0;

    let mut watchdog_counter: u32 = 0;

    let mut recovery_mode = false;

    let mut time_sec: u32 = 0;

    // =====================================
    // SYSTEM HEADER
    // =====================================

    println!();
    println!("====================================");
    println!("   PRESSURE MONITORING SYSTEM");
    println!("        ESP32-S3 + RUST");
    println!("====================================");

    loop {

        let now = Instant::now();

        // =====================================
        // TASK 1:
        // REALTIME ADC SAMPLING
        // =====================================

        if now - previous_sample
            >= sampling_interval
        {
            previous_sample = now;

            pressure_value =
                block!(
                    adc.read_oneshot(
                        &mut pressure_pin
                    )
                )
                .unwrap();

            // =====================================
            // ANOMALY DETECTION
            // =====================================

            if pressure_value < 2000 {

                led.set_high();

                buzzer.set_high();

            } else {

                led.set_low();

                buzzer.set_low();
            }

            // =====================================
            // SOFTWARE WATCHDOG MONITOR
            // =====================================

            if pressure_value <= 300 {

                watchdog_counter += 1;

            } else {

                watchdog_counter = 0;
            }
        }

        // =====================================
        // TASK 2:
        // WATCHDOG RECOVERY
        // =====================================

        if now - previous_watchdog
            >= watchdog_interval
        {
            previous_watchdog = now;

            if watchdog_counter >= 3 {

                recovery_mode = true;

                println!();
                println!("====================================");
                println!("         WATCHDOG ALERT");
                println!("   Sensor Failure Detected");
                println!("   Recovery Mode Activated");
                println!("====================================");

                led.set_high();
                buzzer.set_high();

                delay.delay_millis(1000);

                led.set_low();
                buzzer.set_low();

                watchdog_counter = 0;

            } else {

                recovery_mode = false;
            }
        }

        // =====================================
        // TASK 3:
        // SERIAL MONITOR DISPLAY
        // =====================================

        if now - previous_display
            >= display_interval
        {
            previous_display = now;

            // realtime second counter
            time_sec += 1;

            let pressure_percent =
                (pressure_value as f32
                    / 4095.0)
                    * 100.0;

            println!();

            println!(
                "Time      : {} s",
                time_sec
            );

            println!(
                "ADC Value : {}",
                pressure_value
            );

            println!(
                "Pressure  : {:.2} %",
                pressure_percent
            );

            println!(
                "Status    : {}",
                if pressure_value < 2000 {
                    "ANOMALY"
                } else {
                    "NORMAL"
                }
            );

            println!(
                "LED       : {}",
                if pressure_value < 2000 {
                    "ON"
                } else {
                    "OFF"
                }
            );

            println!(
                "Buzzer    : {}",
                if pressure_value < 2000 {
                    "ON"
                } else {
                    "OFF"
                }
            );

            println!(
                "Recovery  : {}",
                if recovery_mode {
                    "ACTIVE"
                } else {
                    "NORMAL"
                }
            );

        }

        // =====================================
        // SMALL LOOP DELAY
        // =====================================

        delay.delay_millis(10);
    }
}