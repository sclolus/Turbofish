use crate::drivers::pit_8253::PIT0;
use crate::monitor::{Color, SCREEN_MONAD};
use core::time::Duration;

pub fn really_lazy_hello_world() {
    unsafe {
        PIT0.sleep(Duration::from_millis(200));
        SCREEN_MONAD.set_text_color(Color::Green).unwrap();
    }
    print!("H");
    unsafe {
        PIT0.sleep(Duration::from_millis(200));
        SCREEN_MONAD.set_text_color(Color::Red).unwrap();
    }
    print!("E");
    unsafe {
        PIT0.sleep(Duration::from_millis(200));
        SCREEN_MONAD.set_text_color(Color::Blue).unwrap();
    }
    print!("L");
    unsafe {
        PIT0.sleep(Duration::from_millis(200));
        SCREEN_MONAD.set_text_color(Color::Yellow).unwrap();
    }
    print!("L");
    unsafe {
        PIT0.sleep(Duration::from_millis(200));
        SCREEN_MONAD.set_text_color(Color::Cyan).unwrap();
    }
    print!("O");
    unsafe {
        PIT0.sleep(Duration::from_millis(200));
        SCREEN_MONAD.set_text_color(Color::Brown).unwrap();
    }
    print!(" ");
    unsafe {
        PIT0.sleep(Duration::from_millis(200));
        SCREEN_MONAD.set_text_color(Color::Magenta).unwrap();
    }
    print!("W");
    unsafe {
        PIT0.sleep(Duration::from_millis(200));
        SCREEN_MONAD.set_text_color(Color::White).unwrap();
    }
    print!("O");
    unsafe {
        PIT0.sleep(Duration::from_millis(200));
        SCREEN_MONAD.set_text_color(Color::Green).unwrap();
    }
    print!("R");
    unsafe {
        PIT0.sleep(Duration::from_millis(200));
        SCREEN_MONAD.set_text_color(Color::Red).unwrap();
    }
    print!("L");
    unsafe {
        PIT0.sleep(Duration::from_millis(200));
        SCREEN_MONAD.set_text_color(Color::Blue).unwrap();
    }
    print!("D");
    unsafe {
        PIT0.sleep(Duration::from_millis(200));
        SCREEN_MONAD.set_text_color(Color::Yellow).unwrap();
    }
    print!(" ");
    unsafe {
        PIT0.sleep(Duration::from_millis(200));
        SCREEN_MONAD.set_text_color(Color::Cyan).unwrap();
    }
    println!("!");
    unsafe {
        PIT0.sleep(Duration::from_millis(200));
        SCREEN_MONAD.set_text_color(Color::White).unwrap();
    }
}
