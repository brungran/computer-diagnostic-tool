mod cpu;
mod disk;
mod ram;
mod battery;
mod other;

fn main() {
    // cpu::info();
    // disk::info();
    // ram::info();
    // other::mac_addr();
    let _ = battery::health();
    let _ = std::io::stdin().read_line(&mut String::new());
}