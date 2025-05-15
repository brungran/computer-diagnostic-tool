use battery::{Batteries, Battery};

pub fn health() -> Result<(), battery::Error> {
    let manager = battery::Manager::new()?;

    for (idx, maybe_battery) in manager.batteries()?.enumerate() {
        let battery = maybe_battery?;
        println!("Battery #{}:", idx);
        println!("Vendor: {:?}", battery.vendor());
        println!("Model: {:?}", battery.model());
        println!("Time to full charge: {:?}", battery.time_to_full());
        println!("State: {:?}", battery.state());
        println!("State of Charge: {:?}", battery.state_of_charge());
        println!("Health: {:?}", battery.state_of_health().value);
        println!("Voltage: {:.2}", battery.voltage().value);
        println!("Drainage rate: {:.2} W", battery.energy_rate().value);
        println!("Energy: {:?} W", battery.energy().value);
        println!("Energy Full: {:?} W", battery.energy_full().value);
        println!("Energy Full Design: {:?} W", battery.energy_full_design().value);
    }

    Ok(())
}