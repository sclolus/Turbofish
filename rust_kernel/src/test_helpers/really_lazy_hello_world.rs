use crate::drivers::pit_8253::PIT0;
use crate::terminal::Color;
use core::time::Duration;

pub fn really_lazy_hello_world() {
    use Color::*;
    let hello_world = [
        ("H", Green),
        ("E", Red),
        ("L", Blue),
        ("L", Yellow),
        ("O", Cyan),
        (" ", Brown),
        ("W", Magenta),
        ("O", Green),
        ("R", Red),
        ("L", Blue),
        ("D", Yellow),
        (" ", Cyan),
        ("!", White),
    ];
    for (c, color) in hello_world.iter() {
        PIT0.lock().sleep(Duration::from_millis(200));
        set_text_color!(*color);
        print!("{}", c);
    }
    print!("\n");
}
