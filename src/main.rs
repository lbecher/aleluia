mod constants;
mod gui;
mod object;
mod render;

use eframe::{NativeOptions, Result, run_native};
use eframe::egui::ViewportBuilder;
use crate::constants::{GUI_HEIGHT, GUI_WIDTH};
use crate::gui::Gui;

fn main() -> Result {
    env_logger::init();

    let title = "Aleluia";

    let options = NativeOptions {
        viewport: ViewportBuilder::default()
            .with_inner_size([GUI_WIDTH, GUI_HEIGHT]),
        ..Default::default()
    };

    run_native(
        title,
        options,
        Box::new(|_cc| Ok(Box::<Gui>::default())),
    )
}