pub mod hardware;

use hardware::bus::Bus;

#[unsafe(no_mangle)]
pub extern "C" fn add(a: i32, b: i32) -> i32 {
    return a + b;
}

#[unsafe(no_mangle)]
pub extern "C" fn start_emu() {
    let bus: Bus = Bus::new();
}
