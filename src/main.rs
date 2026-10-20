#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use crate::applications::app::Application;
use crate::applications::settings::{WINDOW_HEIGHT, WINDOW_NAME, WINDOW_WIDTH};

use owo_colors::OwoColorize;
use std::process::exit;

pub mod applications;
pub mod extensions;
pub mod workers;

fn main() -> eframe::Result {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_resizable(false)
            .with_inner_size(egui::Vec2::new(WINDOW_WIDTH, WINDOW_HEIGHT)),
        ..Default::default()
    };

    eframe::run_native(
        WINDOW_NAME,
        options,
        Box::new(|cc| Ok(Box::new(Application::new(cc)))),
    )
}

pub fn handle_error(message: &str, mut count: usize, margin: usize) {
    eprintln!("{} {}", "error:".bold().red(), message);
    eprintln!(
        "{} increasing error count by +1 (current counter: {})",
        "warning:".bold().yellow(),
        count
    );

    count += 1;

    if count > margin {
        eprintln!(
            "{} error margin has reached its limit, program will now exit. (current margin: {})",
            "fatal error:".bold().red(),
            margin
        );
        exit(1);
    }
}
