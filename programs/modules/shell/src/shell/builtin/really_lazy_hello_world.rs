use super::ansi_escape_code::color::AnsiColor;

pub fn really_lazy_hello_world() {
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
        unsafe {
            sleep(1);
        }
        print!("{}{}", color, c);
    }
    print!("\n");
}

extern "C" {
    fn sleep(s: u32) -> u32;
}
