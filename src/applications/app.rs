use crate::handle_error;
use crate::workers::thread::{CommunicationSignal, start};

use egui::{FontInsert, InsertFontFamily};

use std::sync::mpsc::{Receiver, Sender, channel};

pub struct Application {
    // clicker data
    interval: u64,
    button: enigo::Button,
    status: bool,

    // fallback protection
    error_count: usize,
    error_margin: usize,

    // communication
    transmitter: Sender<CommunicationSignal>,
}

impl Application {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        add_font(&cc.egui_ctx);

        let (tx, rx): (Sender<CommunicationSignal>, Receiver<CommunicationSignal>) = channel();
        std::thread::spawn(move || start(rx));

        Self {
            // clicker data
            interval: 50,
            button: enigo::Button::Left,
            status: false,

            // fallback protection
            error_count: 0,
            error_margin: 10,

            // communication
            transmitter: tx,
        }
    }
}

impl eframe::App for Application {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ui, |ui| {
            // click interval
            ui.separator();
            ui.heading("click interval");

            let dv_response = ui.add_enabled(!self.status, {
                egui::DragValue::new(&mut self.interval)
                    .range(1..=999)
                    .suffix("ms")
            });

            if dv_response.changed() {
                self.transmitter
                    .send(CommunicationSignal::Interval(self.interval))
                    .unwrap_or_else(|_| {
                        handle_error(
                            "failed to send ”CommunicationSignal::Interval”",
                            self.error_count,
                            self.error_margin,
                        );
                    });
            }

            // click mode
            ui.separator();
            ui.heading("click mode");

            let before = self.button;
            ui.add_enabled_ui(!self.status, |ui| {
                ui.horizontal(|ui| {
                    ui.selectable_value(&mut self.button, enigo::Button::Left, "left");
                    ui.selectable_value(&mut self.button, enigo::Button::Middle, "middle");
                    ui.selectable_value(&mut self.button, enigo::Button::Right, "right");
                })
            });

            if self.button != before {
                self.transmitter
                    .send(CommunicationSignal::Button(self.button))
                    .unwrap_or_else(|_| {
                        handle_error(
                            "failed to send ”CommunicationSignal::Button”",
                            self.error_count,
                            self.error_margin,
                        );
                    });
            }

            // TODO: SHORTCUT KEYS SUPPORT:
            // ui.separator();
            // ui.heading("shortcut keys");

            // click status
            ui.separator();
            ui.heading("click status");

            let status_text;
            if !self.status {
                status_text = "start";
            } else {
                status_text = "stop";
            }

            if ui.button(status_text).clicked() {
                self.status = !self.status;
                self.transmitter
                    .send(CommunicationSignal::Status)
                    .unwrap_or_else(|_| {
                        handle_error(
                            "failed to send “CommunicationSignal::Status”",
                            self.error_count,
                            self.error_margin,
                        );
                    });
            };
        });
    }
}

fn add_font(ctx: &egui::Context) {
    ctx.add_font(FontInsert::new(
        "montserrat-bold",
        egui::FontData::from_static(include_bytes!("../../assets/Montserrat-Bold.ttf")),
        vec![InsertFontFamily {
            family: egui::FontFamily::Proportional,
            priority: egui::epaint::text::FontPriority::Highest,
        }],
    ));
}
