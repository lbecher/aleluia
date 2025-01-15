pub mod parse_input;
pub mod vector_input;

use eframe::{App, Frame};
use eframe::egui::{CentralPanel, Context, SidePanel, TopBottomPanel, Ui, Vec2, Sense, Shape, Rect, Pos2};
use eframe::egui::emath::RectTransform;
use crate::app::vector_input::{vector_input, VectorInputData};
use crate::constants::GUI_SIDEBAR_WIDTH;
use crate::object::Object;
use crate::render::Render;
use crate::types::*;
use crate::utils::*;

pub struct MyApp {
    objects: Vec<Object>,
    selected_object: Option<usize>,

    render: Render,

    vrp: VectorInputData,
    p: VectorInputData,
    y: VectorInputData,
}

impl Default for MyApp {
    fn default() -> Self {
        Self {
            objects: vec![Object::new(10, 10, 3, 3, 20, 20)],
            selected_object: Some(0),

            render: Render::default(),

            vrp: VectorInputData::new(0.0, 0.0, 0.0),
            p: VectorInputData::default(),
            y: VectorInputData::new(0.0, 1.0, 0.0),
        }
    }
}

impl App for MyApp {
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

impl MyApp {
    pub fn menu_bar_content(&mut self, ui: &mut Ui) {
        ui.label("Menu bar");
    }

    pub fn side_panel_content(&mut self, ui: &mut Ui) {
        ui.label("Side panel");

        ui.collapsing("Câmera", |ui| {
            vector_input(ui, "VRP", &mut self.vrp);
            vector_input(ui, "P", &mut self.p);
            vector_input(ui, "Y", &mut self.y);
        });
    }

    pub fn central_panel_content(&mut self, ui: &mut Ui) {
        let painter_size = Vec2::new(ui.available_width(), ui.available_height());
        let painter_sense = Sense::hover();
        let (
            response,
            painter,
        ) = ui.allocate_painter(painter_size, painter_sense);

        if let Some(selected_object) = self.selected_object {
            let to_screen = RectTransform::from_to(
                Rect::from_min_size(Pos2::ZERO, response.rect.size()),
                response.rect,
            );

            let control_point_radius = 8.0;
            let control_point_shapes: Vec<Shape> = self.objects[selected_object].control_points
                .iter_mut()
                .enumerate()
                .map(|(i, point_sru)| {
                    let m_sru_srt: Mat4 = self.render.get_m_sru_srt();

                    let mut point_srt: Mat4x1 = m_sru_srt * *point_sru;
                    let mut point_srt_pos2 = Pos2::new(point_srt.x, point_srt.y);

                    let size = Vec2::splat(2.0 * control_point_radius);

                    let point_in_screen = to_screen.transform_pos(point_srt_pos2);
                    let point_rect = Rect::from_center_size(point_in_screen, size);
                    let point_id = response.id.with(i);
                    let point_response = ui.interact(point_rect, point_id, Sense::drag());

                    let drag_delta = point_response.drag_delta();

                    if drag_delta != Vec2::ZERO {
                        let drag_delta_srt: Mat4x1 = Mat4x1::new(drag_delta.x, drag_delta.y, 0.0, 0.0);
                        let drag_delta_sru: Mat4x1 = m_sru_srt.try_inverse().unwrap() * drag_delta_srt;

                        translate(&mut point_srt, drag_delta_srt.x, drag_delta_srt.y, drag_delta_srt.z);
                        translate(&mut *point_sru, drag_delta_sru.x, drag_delta_sru.y, drag_delta_sru.z);

                        point_srt_pos2 = Pos2::new(point_srt.x, point_srt.y);

                        /*point_pos2 += point_response.drag_delta();
                        point_pos2 = to_screen.from().clamp(point_pos2);

                        point_sru.x = point_pos2.x;
                        point_sru.y = point_pos2.y;

                        *point_sru = m_sru_srt.try_inverse().unwrap() * *point_sru;*/
                    }

                    //point_srt_pos2 = to_screen.from().clamp(point_srt_pos2);

                    let point_in_screen = Pos2::new(point_srt.x, point_srt.y);
                    let stroke = ui.style().interact(&point_response).fg_stroke;

                    Shape::circle_stroke(point_in_screen, control_point_radius, stroke)
                })
                .collect();

            /*let points_in_screen: Vec<Pos2> = control_points
                .iter()
                .map(|p| to_screen * *p)
                .collect();*/

            painter.extend(control_point_shapes);
        }
    }
}