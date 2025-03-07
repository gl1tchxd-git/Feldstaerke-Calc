# Rust Coil Calculator

A Rust library for calculating the magnetic field of different types of electric coils using the Biot-Savart law.

## Usage

Add this to your `Cargo.toml`:

```toml
[dependencies]
rust-coil-calculator = "0.1.0"
```

Then, in your code:

```rust
use rust_coil_calculator::coil::Coil;
use rust_coil_calculator::solenoid::Solenoid;
use rust_coil_calculator::toroidal::Toroidal;
use rust_coil_calculator::circular::Circular;

let solenoid = Solenoid { current: 1.0, turns: 100, length: 0.1, radius: 0.01 };
let coil = Coil::Solenoid(solenoid);
let field = coil.calculate_field((0.0, 0.0, 0.1));
```

## License

This project is licensed under the MIT License.