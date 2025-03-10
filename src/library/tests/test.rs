#[cfg(test)]
mod tests {
    use coil_calculator::coil::Coil;
    use coil_calculator::solenoid::Solenoid;

    #[test]
    fn solenoid () {
        let solenoid = Solenoid {
            current: 1.5,
            turns: 100,
            length: 0.2,
            radius: 0.01,
            wire_radius: 0.001,
        };
        assert_eq!(Coil::Solenoid(solenoid).calculate_field(4000), 373.47034389680505);
    }
}