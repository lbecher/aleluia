pub mod parse_input;
pub mod vector_input;

use eframe::{App, Frame};
use eframe::egui::{CentralPanel, Context, SidePanel, TopBottomPanel};
use crate::constants::GUI_SIDEBAR_WIDTH;
use crate::gui::vector_input::vector_input;

pub struct Gui {
    vrp_x: f32,
    vrp_y: f32,
    vrp_z: f32,
    vrp_xs: String,
    vrp_ys: String,
    vrp_zs: String,
}

impl Default for Gui {
    fn default() -> Self {
        Self {
            vrp_x: 0.0,
            vrp_y: 0.0,
            vrp_z: 0.0,
            vrp_xs: "X: 0".to_string(),
            vrp_ys: "Y: 0".to_string(),
            vrp_zs: "Z: 0".to_string(),
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

        vector_input(
            ui,
            "VRP",
            &mut self.vrp_xs,
            &mut self.vrp_ys,
            &mut self.vrp_zs,
            &mut self.vrp_x,
            &mut self.vrp_y,
            &mut self.vrp_z,
        );
    }

    pub fn central_panel_content(&mut self, ui: &mut eframe::egui::Ui) {
        ui.label("Central panel");
    }
}