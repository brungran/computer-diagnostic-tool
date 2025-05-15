use sysinfo::{System, Networks};
use chrono::{DateTime, Utc,};
use chrono_tz::America::Sao_Paulo;

pub fn uptime(){
    println!("System running for {:.2} hours.", System::uptime() as f64 / 3600.0);
}

pub fn time_booted(){
    let boot_timestamp_seconds = i64::try_from(System::boot_time())
    .expect("Boot timestamp too big to convert.");
    // Data UTC
    let booted_at_utc = DateTime::<Utc>::from_timestamp(boot_timestamp_seconds, 0)
    .expect("Invalid timestamp");
    // Converter para horário de Brasília
    let booted_at_br = booted_at_utc.with_timezone(&Sao_Paulo);
    println!("System booted at {}", booted_at_br.format("%d/%m/%Y %H:%M"));
}

pub fn hostname(){
    println!("{:?}", System::host_name());
}

pub fn os(){
    println!("{:?}", System::long_os_version());
}

pub fn process_list(){
    let s = System::new_all();
    for (pid, process) in s.processes() {
        println!("{} {:?}", pid, process.name());
    }
}

pub fn mac_addr(){
    let mut networks = Networks::new_with_refreshed_list();
    for (interface_name, network) in &networks {
        println!("MAC address: {}", network.mac_address());
    }
}