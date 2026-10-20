use enigo::{Enigo, Mouse};

pub fn send_button(button: enigo::Button, enigo: &mut Enigo) {
    enigo
        .button(button, enigo::Direction::Click)
        .unwrap_or_else(|e| {
            eprintln!("fatal error: failed to simulate send button ({})", e);
            panic!();
        });
}

pub fn release_button(button: enigo::Button, enigo: &mut Enigo) {
    enigo
        .button(button, enigo::Direction::Release)
        .unwrap_or_else(|e| {
            eprintln!("fatal error: failed to simulate release button ({})", e);
            panic!();
        });
}
