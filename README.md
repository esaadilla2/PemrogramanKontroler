# Interrupt-Based Realtime Pipeline Pressure Anomaly Detection with Watchdog Recovery on ESP32-S3

---

# Project Overview

This project implements a realtime embedded pressure monitoring and anomaly detection system using ESP32-S3 and Rust programming language.

The system continuously performs:

- realtime ADC sampling
- pressure level monitoring
- anomaly detection
- leak simulation detection
- watchdog monitoring
- fault recovery handling
- buzzer and LED alert activation

The project is designed as an embedded realtime system simulation prototype for industrial instrumentation and control applications.

---

# Objectives

The objectives of this project are:

- Implement realtime pressure monitoring on ESP32-S3
- Simulate pipeline pressure anomaly detection
- Implement software watchdog recovery mechanism
- Develop embedded realtime task scheduling using Rust
- Simulate industrial fault detection behavior

---

# Main Features

- Realtime ADC sampling
- Pressure normalization into percentage scale
- Leak detection simulation
- Software watchdog recovery
- Realtime serial monitoring
- LED warning indicator
- Buzzer alarm system
- Wokwi hardware simulation
- Rust embedded-hal implementation

---

# System Architecture

The system is divided into several realtime software tasks.

| Task | Description | Interval |
|---|---|---|
| ADC Sampling Task | Reads analog pressure input | 100 ms |
| Display Task | Displays monitoring data | 1000 ms |
| Watchdog Monitoring Task | Detects sensor abnormal condition | 3000 ms |

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
Pressure Normalization
      ↓
Anomaly Detection
      ↓
LED/Buzzer Alert
      ↓
Watchdog Monitoring
      ↓
Recovery Mechanism
```

---

# Pressure Simulation

Pressure input is simulated using potentiometer ADC mapping.

ADC values are normalized into percentage scale using:

```rust
pressure_percent =
    (pressure_value as f32 / 4095.0) * 100.0;
```

---

# ADC Mapping Table

| ADC Value | Pressure Level |
|---|---|
| 0 | 0 % |
| 1024 | 25 % |
| 2047 | 50 % |
| 3071 | 75 % |
| 4095 | 100 % |

---

# Realtime Sampling

The system samples ADC input every:

```text
100 ms
```

This means the system performs:

```text
10 ADC readings per second
```

---

# Display Update

Serial monitor output updates every:

```text
1000 ms
```

This avoids excessive terminal spam while maintaining realtime monitoring behavior.

---

# Pressure Anomaly Detection

The system detects abnormal pressure drops using threshold comparison.

```rust
if pressure_value < 2000
```

If pressure falls below threshold:

- LED turns ON
- Buzzer turns ON
- Status changes to ANOMALY

This simulates sudden pipeline pressure loss or leak condition.

---

# Software Watchdog Recovery

The watchdog mechanism detects sensor abnormal conditions.

```rust
if pressure_value <= 300
```

If abnormal condition persists for several monitoring cycles:

- Recovery mode activates
- Watchdog alert appears
- Alarm system triggers

This simulates fault handling and embedded recovery behavior.

---

# Simulation Scenarios

---

# 1. Normal Condition

## Description

Potentiometer is set to medium or high value.

## Expected Behavior

- Stable pressure value
- Status = NORMAL
- LED OFF
- Buzzer OFF
- Recovery = NORMAL

## Example Data

| Time (s) | Pressure (%) |
|---|---|
| 1 | 80 |
| 2 | 79 |
| 3 | 81 |
| 4 | 78 |
| 5 | 80 |

---

# 2. Leak Detection Simulation

## Description

Potentiometer value is suddenly decreased.

## Expected Behavior

- Sudden pressure drop detected
- Status = ANOMALY
- LED ON
- Buzzer ON

## Example Data

| Time (s) | Pressure (%) |
|---|---|
| 1 | 80 |
| 2 | 79 |
| 3 | 78 |
| 4 | 35 |
| 5 | 20 |

---

# 3. Watchdog Recovery Simulation

## Description

Potentiometer is reduced near 0%.

## Expected Behavior

- Sensor abnormal condition detected
- Watchdog recovery activated
- Recovery status becomes ACTIVE

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

The terminal displays:

- Time
- ADC value
- Pressure percentage
- Status condition
- Recovery state

in realtime.



Instrumentation Engineering

ESP32-S3 Embedded Realtime System Project
