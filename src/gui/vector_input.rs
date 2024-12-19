use eframe::egui::{TextEdit, Ui};
use crate::constants::GUI_VECTOR_INPUT_WIDTH;
use crate::gui::parse_input::parse_input;

pub fn vector_input(
    ui: &mut Ui,
    label: &str,
    xs: &mut String,
    ys: &mut String,
    zs: &mut String,
    x: &mut f32,
    y: &mut f32,
    z: &mut f32,
) {
    ui.collapsing(label, |ui| {
        ui.horizontal(|ui| {
            ui.add(TextEdit::singleline(xs)
                .desired_width(GUI_VECTOR_INPUT_WIDTH));
            ui.add(TextEdit::singleline(ys)
                .desired_width(GUI_VECTOR_INPUT_WIDTH));
            ui.add(TextEdit::singleline(zs)
                .desired_width(GUI_VECTOR_INPUT_WIDTH));

            if ui.button("Aplicar").clicked() {
                parse_input("X:", x, xs);
                parse_input("Y:", y, ys);
                parse_input("Z:", z, zs);
            }
        });
    });
}

