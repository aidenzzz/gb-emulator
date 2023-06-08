use emulator;

fn main() {
    println!("Hello, world!");
    println!("2 + 2 = {}", emulator::add(2, 2));
    let test: u16 = 0x80;
    println!("test: {:02x}", test as u16);
}