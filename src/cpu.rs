use sysinfo::{System, RefreshKind, CpuRefreshKind};

pub fn info() {
    let mut s = System::new_with_specifics(
        RefreshKind::nothing().with_cpu(CpuRefreshKind::everything()),
    );
    
    // Wait a bit because CPU usage is based on diff.
    std::thread::sleep(sysinfo::MINIMUM_CPU_UPDATE_INTERVAL);

    // Refresh CPUs again to get actual value.
    s.refresh_cpu_all();

    println!("{} {}", s.cpus()[0].brand(), System::cpu_arch());
    println!("Global Usage: {}%", s.global_cpu_usage());
    println!("Usage per Core:");
    println!("Name\tUsage (%)\tFrequency (MHz)");
    for cpu in s.cpus() {
        println!("{}\t{}%\t{}", cpu.name(), cpu.cpu_usage(), cpu.frequency());
    }
}
