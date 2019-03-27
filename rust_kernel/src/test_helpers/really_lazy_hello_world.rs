use crate::drivers::pit_8253::PIT0;
use crate::monitor::{Color, SCREEN_MONAD};
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
        SCREEN_MONAD.lock().set_text_color(*color).unwrap();
        print!("{}", c);
    }
    print!("\n");
}
