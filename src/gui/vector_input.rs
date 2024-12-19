use eframe::egui::{TextEdit, Ui};
use crate::constants::GUI_VECTOR_INPUT_WIDTH;
use crate::gui::parse_input::parse_input;

pub struct VectorInput {
    pub xv: f32,
    pub yv: f32,
    pub zv: f32,
    pub xs: String,
    pub ys: String,
    pub zs: String,
}

impl Default for VectorInput {
    fn default() -> Self {
        Self {
            xv: 0.0,
            yv: 0.0,
            zv: 0.0,
            xs: "X: 0".to_string(),
            ys: "Y: 0".to_string(),
            zs: "Z: 0".to_string(),
        }
    }
}

pub fn vector_input(
    ui: &mut Ui,
    label: &str,
    input: &mut VectorInput,
) {
    ui.collapsing(label, |ui| {
        ui.horizontal(|ui| {
            ui.add(TextEdit::singleline(&mut input.xs)
                .desired_width(GUI_VECTOR_INPUT_WIDTH));
            ui.add(TextEdit::singleline(&mut input.ys)
                .desired_width(GUI_VECTOR_INPUT_WIDTH));
            ui.add(TextEdit::singleline(&mut input.zs)
                .desired_width(GUI_VECTOR_INPUT_WIDTH));

            if ui.button("Aplicar").clicked() {
                parse_input("X:", &mut input.xv, &mut input.xs);
                parse_input("Y:", &mut input.yv, &mut input.ys);
                parse_input("Z:", &mut input.zv, &mut input.zs);
            }
        });
    });
}

