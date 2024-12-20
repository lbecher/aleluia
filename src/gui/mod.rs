pub mod parse_input;
pub mod vector_input;

use eframe::{App, Frame};
use eframe::egui::{CentralPanel, Context, SidePanel, TopBottomPanel};
use crate::constants::GUI_SIDEBAR_WIDTH;
use crate::gui::vector_input::{vector_input, VectorInputData};



pub struct Gui {
    vrp: VectorInputData,
    p: VectorInputData,
    y: VectorInputData,
}

impl Default for Gui {
    fn default() -> Self {
        Self {
            vrp: VectorInputData::new(0.0, 0.0, 0.0),
            p: VectorInputData::default(),
            y: VectorInputData::new(0.0, 1.0, 0.0),
        }
    }
}

impl App for Gui {
    fn update(&mut self, ctx: &Context, _frame: &mut Frame) {
        TopBottomPanel::top("menu_bar")
            .show(ctx, |ui| {
                self.menu_bar_content(ui);
            });

        SidePanel::right("side_panel")
            .exact_width(GUI_SIDEBAR_WIDTH)
            .resizable(false)
            .show(ctx,  |ui| {
                self.side_panel_content(ui);
            });

        CentralPanel::default()
            .show(ctx, |ui| {
                self.central_panel_content(ui);
            });
    }
}

impl Gui {
    pub fn menu_bar_content(&mut self, ui: &mut eframe::egui::Ui) {
        ui.label("Menu bar");
    }

    pub fn side_panel_content(&mut self, ui: &mut eframe::egui::Ui) {
        ui.label("Side panel");

        ui.collapsing("Câmera", |ui| {
            vector_input(ui, "VRP", &mut self.vrp);
            vector_input(ui, "P", &mut self.p);
            vector_input(ui, "Y", &mut self.y);
        });
    }

    pub fn central_panel_content(&mut self, ui: &mut eframe::egui::Ui) {
        ui.label("Central panel");
    }
}