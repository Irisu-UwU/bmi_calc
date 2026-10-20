#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use eframe::egui::{self, Slider};

fn main() -> eframe::Result {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([320.0, 240.0]),
        ..Default::default()
    };
    eframe::run_native(
        "BMI Calculator",
        options,
        Box::new(|_| {
            Ok(Box::<UserInfo>::default())
        }),
    )
}

struct UserInfo {
    weight: f64,
    height: f64
}

impl Default for UserInfo {
    fn default() -> Self {
        Self {
            weight: 75.0,
            height: 175.0,
        }
    }
}

impl eframe::App for UserInfo {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ui, |ui| {
            ui.heading("BMI Calculator");
            ui.horizontal(|ui| {
                let _ = ui.label("Your weight (in kg): ");
                // ui.add(egui::DragValue::new(&mut self.weight));
                ui.add(Slider::new(&mut self.weight, 0.0..=300.0)
                    .fixed_decimals(1).step_by(1.0).smart_aim(false).clamping(egui::SliderClamping::Never));
            });
            ui.horizontal(|ui| {
                let _ = ui.label("Your height (in cm): ");
                ui.add(Slider::new(&mut self.height, 0.0..=300.0)
                    .fixed_decimals(1).step_by(1.0).smart_aim(false).clamping(egui::SliderClamping::Never));
            });
            ui.horizontal(|_| {});
            ui.horizontal(|_| {});
            ui.separator();
            ui.horizontal(|_| {});
            ui.horizontal(|_| {});

            let bmi = self.weight / self.height.powf(2.0) * 10000.0;
            ui.label(format!("Your Body Mass Index is: {:.1}", bmi));
            let bmi_stat = match bmi {
                ..18.5 => "Underweight",
                18.5..25.0 => "Normal weight",
                25.0..30.0 => "Overweight",
                _ => "Obese"
            };
            ui.label(format!("Your BMI status is: {bmi_stat}"));
        });
    }
}