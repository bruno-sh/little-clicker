use crate::extensions::button_extension::ButtonToU8;
use crate::extensions::button_extension::U8ToButton;

use crate::workers::input::release_button;
use crate::workers::input::send_button;

use enigo::{Enigo, Settings};
use owo_colors::OwoColorize;

use std::process::exit;

use std::sync::Arc;
use std::sync::atomic::{AtomicBool, AtomicU8, AtomicU64, Ordering};
use std::sync::mpsc::Receiver;

use std::time::Duration;

pub enum CommunicationSignal {
    Interval(u64),
    Button(enigo::Button),
    Status,
}

struct SharedState {
    interval: AtomicU64,
    button: AtomicU8,
    status: AtomicBool,
}

pub fn start(rx: Receiver<CommunicationSignal>) {
    let state = Arc::new(SharedState {
        interval: AtomicU64::new(50),
        button: AtomicU8::new(1),
        status: AtomicBool::new(false),
    });

    let clicker_state = Arc::clone(&state);
    start_clicker(clicker_state);

    let receiver_state = Arc::clone(&state);
    capture_signal(rx, receiver_state);
}

fn start_clicker(state: Arc<SharedState>) {
    // cannot continue without enigo
    let mut enigo = Enigo::new(&Settings::default()).unwrap_or_else(|e| {
        eprintln!(
            "{} failed to create enigo ({e})",
            "fatal error:".bold().red()
        );
        exit(1);
    });

    std::thread::spawn(move || {
        loop {
            if state.status.load(Ordering::Relaxed) {
                let button_value = state.button.load(Ordering::Relaxed);
                let button = button_value.to_button();

                send_button(button, &mut enigo);
                release_button(button, &mut enigo);
            }

            let interval = state.interval.load(Ordering::Relaxed);
            std::thread::sleep(Duration::from_millis(interval));
        }
    });
}

fn capture_signal(rx: Receiver<CommunicationSignal>, state: Arc<SharedState>) {
    std::thread::spawn(move || {
        loop {
            // cannot continue without receiving the signal
            let signal = rx.recv().unwrap_or_else(|e| {
                eprintln!(
                    "{} failed to create enigo ({e})",
                    "fatal error:".bold().red()
                );
                exit(1);
            });

            match signal {
                CommunicationSignal::Interval(interval) => {
                    state.interval.store(interval, Ordering::Relaxed);
                }
                CommunicationSignal::Button(button) => {
                    let v = button.to_u8();
                    state.button.store(v, Ordering::Relaxed);
                }
                CommunicationSignal::Status => {
                    let status = state.status.load(Ordering::Relaxed);
                    state.status.store(!status, Ordering::Relaxed);
                }
            }
        }
    });
}
