use sysinfo::{System, SystemExt, CpuExt};

fn main() {
    println!("Coletando dados do sistema...");

    let mut sys = System::new_all();
    sys.refresh_all();

    // CPU
    let cpu_usage = sys.global_cpu_info().cpu_usage();
    let cpu_score = if cpu_usage < 70.0 {
        25
    } else if cpu_usage < 90.0 {
        15
    } else {
        5
    };

    // Memória
    let total_mem = sys.total_memory() as f64;
    let used_mem = sys.used_memory() as f64;
    let mem_percent = (used_mem / total_mem) * 100.0;
    let mem_score = if mem_percent < 70.0 {
        25
    } else if mem_percent < 90.0 {
        15
    } else {
        5
    };

    let total_score = cpu_score + mem_score;

    println!();
    println!("[CPU] Uso: {:.1}% — Score: {}/25", cpu_usage, cpu_score);
    println!("[RAM] Uso: {:.1}% — Score: {}/25", mem_percent, mem_score);
    println!("→ Score total: {}/50", total_score);
}
