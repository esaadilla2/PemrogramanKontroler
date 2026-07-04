# Interrupt-Based Real-Time Pipeline Pressure Monitoring and Anomaly Detection Using ESP32-S3

---

# Project Report

The complete project report & PPT can be accessed through the Google Drive link below:

[📑 REPORT](https://drive.google.com/file/d/1rsivgzsf5GSL6BsVzhmnbfy5t2P1h9J3/)

[💻 PPT](https://canva.link/5qzsiky4l29eidn)

## Presentation

The project was presented and demonstrated as part of the Programming Controller course evaluation.

![Project Presentation](images/presentation.png)

# Project Overview

This project implements a realtime embedded pressure monitoring and anomaly detection system using ESP32-S3 and Rust programming language.

The system continuously performs:

- Realtime ADC sampling
- Pressure monitoring
- Pressure calculation
- Pressure anomaly detection
- Leak simulation using potentiometer
- LED and buzzer alarm activation
- UART serial monitoring

The project is designed as an embedded realtime system simulation prototype for industrial instrumentation and control applications.

---

# Objectives

The objectives of this project are:

- Implement realtime pressure monitoring on ESP32-S3
- Implement pressure calculation based on ADC conversion
- Implement real-time pressure anomaly detection
- Develop interrupt-based embedded task scheduling using Rust
- Simulate pipeline leak detection using a potentiometer
  
---

# Main Features

- Realtime ADC sampling
- ADC-to-voltage conversion
- Voltage-to-pressure conversion
- Pressure threshold-based anomaly detection
- UART serial monitoring
- LED warning indicator
- Buzzer alarm system
- Anomaly counter
- Wokwi hardware simulation
- Rust esp-hal implementation

---

# System Architecture

The system is divided into several realtime software tasks.

| Task | Description | Interval |
|---|---|---|
| ADC Sampling Task | Read ADC and calculate pressure | 100 ms |
| Display Task | Displays monitoring data | 1000 ms |

---

# Hardware Components

| Component | Function |
|---|---|
| ESP32-S3 | Main microcontroller |
| Potentiometer | Simulated pressure sensor |
| LED | Visual anomaly indicator |
| Buzzer | Audio warning system |

---

# Software Stack

| Software | Purpose |
|---|---|
| Rust | Embedded programming |
| esp-hal | ESP32 hardware abstraction layer |
| probe-rs | Flashing/debugging |
| Wokwi | Hardware simulation |
| VSCode | Development environment |

---

# Pin Configuration

| Component | ESP32-S3 Pin |
|---|---|
| Potentiometer Signal | GPIO4 |
| LED | GPIO2 |
| Buzzer | GPIO5 |

---

# Wokwi Wiring Configuration

## Potentiometer

| Potentiometer Pin | ESP32-S3 |
|---|---|
| VCC | 3.3V |
| GND | GND |
| SIG | GPIO4 |

---

## LED

| LED Pin | ESP32-S3 |
|---|---|
| Anode (+) | GPIO2 |
| Cathode (-) | GND |

---

## Buzzer

| Buzzer Pin | ESP32-S3 |
|---|---|
| Signal / VCC | GPIO5 |
| GND | GND |

---

# Realtime Pipeline Mechanism

The system uses periodic software scheduling to simulate realtime embedded task execution.

## Pipeline Flow

```text
ADC Sampling
      ↓
ADC to Voltage Conversion
      ↓
Voltage to Pressure Conversion
      ↓
Pressure Threshold Detection
      ↓
Anomaly Detection & Counter
      ↓
LED/Buzzer Alarm
      ↓
UART Monitoring

```

---

# Pressure Simulation

The potentiometer is used as an analog input to simulate the output voltage of a pressure sensor. The ESP32-S3 reads the analog voltage through its 12-bit ADC, producing a digital value in the range of 0–4095.

The ADC value is first converted into an analog voltage using:

```text
Vin = (ADC / 4095) × VREF
```

where:

- ADC = 12-bit ADC reading (0–4095)
- 4095 = Maximum value of a 12-bit ADC
- VREF = ADC reference voltage (3.3 V)

After obtaining the input voltage, the pressure is calculated using the MPX5010 transfer function:

```text
Pressure = ((Vin / VS) - 0.04) / 0.09
```

where:

- Pressure = Calculated pressure (kg/cm²)
- Vin = Voltage obtained from the ADC conversion
- VS = Sensor supply voltage (5 V)

The calculated pressure is then compared with a predefined threshold.

```rust
let vin =
    (pressure_raw as f32 / 4095.0)
    * VREF;

pressure =
    ((vin / VS) - 0.04)
    / 0.09;
```

If the pressure value falls below the predefined threshold, the system identifies the condition as a pressure anomaly (pipeline leak simulation), activates the LED and buzzer, and increments the anomaly counter.


---

# Real-Time Sampling

The system performs periodic ADC sampling every:

```text
100 ms
```

This corresponds to:

```text
10 ADC readings per second
```

Periodic sampling ensures that pressure data are acquired consistently for real-time monitoring and anomaly detection.

---

# Display Update

The monitoring results are transmitted to the serial monitor every:

```text
1000 ms
```

The displayed information includes:

- Time
- ADC Raw Value
- Pressure (kg/cm²)
- Pressure Threshold
- Anomaly Counter
- System Status
- LED Status
- Buzzer Status

Updating the display once every second improves readability while maintaining real-time monitoring performance.

---

# Pressure Anomaly Detection

Pressure anomaly detection is performed using a predefined pressure threshold.

```rust
if pressure < PRESSURE_THRESHOLD
```

where

```rust
const PRESSURE_THRESHOLD: f32 = 5.0;
```

If the calculated pressure falls below the threshold:

- LED turns ON
- Buzzer turns ON
- Status changes to **ANOMALY**
- Anomaly Counter increases

Otherwise:

- LED turns OFF
- Buzzer turns OFF
- Status changes to **NORMAL**
- Anomaly Counter resets to zero

This simulates a sudden pressure drop that may indicate a pipeline leak.

---

# Anomaly Counter

The anomaly counter records the number of consecutive pressure readings below the predefined threshold.

```rust
anomaly_counter += 1;
```

If the pressure returns to the normal operating range, the counter is automatically reset.

```rust
anomaly_counter = 0;
```

The counter is used to indicate how long an abnormal pressure condition persists during system operation.

---

# Simulation Scenarios

---

# 1. Normal Condition

## Description

The potentiometer is adjusted so that the calculated pressure remains above the predefined threshold (5.00 kg/cm²).

## Expected Behavior

- Pressure > 5.00 kg/cm²
- Status = NORMAL
- LED = OFF
- Buzzer = OFF
- Anomaly Counter = 0

## Example Data

| Time (s) | ADC Raw | Pressure (kg/cm²) | Status | LED | Buzzer | Anomaly Counter |
|----------|--------:|------------------:|--------|-----|--------|----------------:|
| 1 | 3767 | 6.30 | NORMAL | OFF | OFF | 0 |
| 2 | 3787 | 6.34 | NORMAL | OFF | OFF | 0 |
| 3 | 4095 | 6.89 | NORMAL | OFF | OFF | 0 |
| 4 | 3928 | 6.59 | NORMAL | OFF | OFF | 0 |
| 5 | 3993 | 6.71 | NORMAL | OFF | OFF | 0 |
| 6 | 4095 | 6.89 | NORMAL | OFF | OFF | 0 |
| 7 | 3727 | 6.23 | NORMAL | OFF | OFF | 0 |
| 8 | 4095 | 6.89 | NORMAL | OFF | OFF | 0 |
| 9 | 3752 | 6.27 | NORMAL | OFF | OFF | 0 |
| 10 | 4095 | 6.89 | NORMAL | OFF | OFF | 0 |

---

# 2. Pressure Anomaly (Leak Simulation)

## Description

The potentiometer is adjusted to simulate a pressure drop below the predefined threshold (5.00 kg/cm²).

## Expected Behavior

- Pressure < 5.00 kg/cm²
- Status = ANOMALY
- LED = ON
- Buzzer = ON
- Anomaly Counter increases continuously while the pressure remains below the threshold.

## Example Data

| Time (s) | ADC Raw | Pressure (kg/cm²) | Status | LED | Buzzer | Anomaly Counter |
|----------|--------:|------------------:|--------|-----|--------|----------------:|
| 1 | 3470 | 5.77 | NORMAL | OFF | OFF | 0 |
| 2 | 2889 | 4.73 | ANOMALY | ON | ON | 2 |
| 3 | 2659 | 4.32 | ANOMALY | ON | ON | 12 |
| 4 | 2456 | 3.95 | ANOMALY | ON | ON | 22 |
| 5 | 2223 | 3.54 | ANOMALY | ON | ON | 32 |
| 6 | 2078 | 3.28 | ANOMALY | ON | ON | 42 |
| 7 | 2036 | 3.20 | ANOMALY | ON | ON | 52 |
| 8 | 2079 | 3.28 | ANOMALY | ON | ON | 62 |
| 9 | 2087 | 3.29 | ANOMALY | ON | ON | 72 |
| 10 | 2088 | 3.29 | ANOMALY | ON | ON | 82 |
---

# Project Structure

```text
.
├── src/
│   ├── bin/
│   │   └── main.rs
│   └── lib.rs
├── diagram.json
├── wokwi.toml
├── Cargo.toml
├── Cargo.lock
├── build.rs
├── rust-toolchain.toml
└── README.md
```

---

# How to Run

---

# 1. Clone Repository

```bash
git clone https://github.com/esaadilla2/PemrogramanKontroler.git
```

---

# 2. Open Project

Open the project using:

```text
VSCode
```

---

# 3. Install Requirements

Install the following tools:

- Rust toolchain
- espup
- probe-rs
- Wokwi VSCode Extension

---

# 4. Open Wokwi Simulation

Open:

```text
diagram.json
```

and:

```text
wokwi.toml
```

---

# 5. Run Simulation

Run using:

```bash
cargo run
```

---

# 6. Monitor Serial Output

The serial monitor displays the following information in real time:

- Time (s)
- ADC Raw Value
- Calculated Pressure (kg/cm²)
- Pressure Threshold (kg/cm²)
- Anomaly Counter
- System Status (NORMAL / ANOMALY)
- LED Status (ON / OFF)
- Buzzer Status (ON / OFF)

Example output:

```text
====================================
   PRESSURE MONITORING SYSTEM
        ESP32-S3 + RUST
Threshold : 5.00 kg/cm2
====================================

Time         : 1 s
ADC Raw      : 3767
Pressure     : 6.30 kg/cm2
Threshold    : 5.00 kg/cm2
Anomaly Cnt  : 0
Status       : NORMAL
LED          : OFF
Buzzer       : OFF
```



Instrumentation Engineering

ESP32-S3 Embedded Realtime System Project
