use crate::drivers::PIT0;
use ansi_escape_code::color::AnsiColor;
use core::time::Duration;

pub fn really_lazy_hello_world(duration: Duration) {
    let hello_world = [
        ("H", AnsiColor::GREEN),
        ("E", AnsiColor::RED),
        ("L", AnsiColor::BLUE),
        ("L", AnsiColor::YELLOW),
        ("O", AnsiColor::CYAN),
        (" ", AnsiColor::WHITE),
        ("W", AnsiColor::MAGENTA),
        ("O", AnsiColor::GREEN),
        ("R", AnsiColor::RED),
        ("L", AnsiColor::BLUE),
        ("D", AnsiColor::YELLOW),
        (" ", AnsiColor::WHITE),
        ("!", AnsiColor::WHITE),
    ];
    for (c, color) in hello_world.iter() {
        PIT0.lock().sleep(duration);
        print!("{}{}", color, c);
    }
    print!("\n");
}
