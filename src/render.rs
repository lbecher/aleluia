use std::collections::BTreeMap;

use eframe::egui::viewport;
use ordered_float::OrderedFloat;
use rayon::prelude::*;
use crate::camera;
use crate::types::{Mat4, Mat4x1, Vec3};
use crate::utils::{mat4x1_to_vec3, vec3_to_mat4x1};

#[derive(Clone, PartialEq)]
pub enum ProjectionType {
    Orthographic,
    Perspective,
}

#[derive(Clone, PartialEq)]
pub enum ShaderType {
    Wireframe,
    Constant,
    Gouraud,
    Phong,
}

pub struct Camera {
    pub vrp: Vec3,
    pub p: Vec3,
    pub y: Vec3,
    pub dp: f32,
}

pub struct Window {
    pub xmin: f32,
    pub xmax: f32,
    pub ymin: f32,
    pub ymax: f32,
}

pub struct Viewport {
    pub umin: f32,
    pub umax: f32,
    pub vmin: f32,
    pub vmax: f32,
}

pub struct Render {
    shader_type: ShaderType,
    projection_type: ProjectionType,
    camera: Camera,
    window: Window,
    viewport: Viewport,
    m_sru_srt: Mat4,
}

impl Default for Render {
    fn default() -> Self {
        let projection_type = ProjectionType::Orthographic;
        let shader_type = ShaderType::Wireframe;

        let camera = Camera {
            vrp: Vec3::new(25.0, 15.0, 80.0),
            p: Vec3::new(20.0, 10.0, 25.0),
            y: Vec3::new(0.0, 1.0, 0.0),
            dp: 40.0,
        };
        let window = Window {
            xmin: -20.0,
            xmax: 20.0,
            ymin: -15.0,
            ymax: 15.0,
        };
        let viewport = Viewport {
            umin: 0.0,
            umax: 299.0,
            vmin: 0.0,
            vmax: 199.0,
        };

        let m_sru_srt: Mat4 = match projection_type {
            ProjectionType::Orthographic => Render::calc_sru_srt_orth_matrix(&camera, &window, &viewport),
            ProjectionType::Perspective => Render::calc_sru_srt_pers_matrix(&camera, &window, &viewport),
        };

        let render = Self {
            projection_type,
            shader_type,
            camera,
            window,
            viewport,
            m_sru_srt,
        };

        render
    }
}

impl Render {
    pub fn set_camera(&mut self, camera: Camera) {
        self.camera = camera;
    }

    pub fn set_window(&mut self, window: Window) {
        self.window = window;
    }

    pub fn set_viewport(&mut self, viewport: Viewport) {
        self.viewport = viewport;
    }

    pub fn set_projection(
        &mut self,
        camera: &Camera,
        projection_type: ProjectionType,
    ) {
        self.m_sru_srt = match projection_type {
            ProjectionType::Orthographic => Render::calc_sru_srt_orth_matrix(&camera, &self.window, &self.viewport),
            ProjectionType::Perspective => Render::calc_sru_srt_pers_matrix(&camera, &self.window, &self.viewport),
        };
        self.projection_type = projection_type;
    }

    pub fn set_shader(&mut self, shader_type: ShaderType) {
        self.shader_type = shader_type;
    }

    pub fn get_m_sru_srt(&self) -> Mat4 {
        self.m_sru_srt
    }

    #[inline(always)]
    fn calc_sru_src_matrix(camera: &Camera, nn: &Vec3) -> Mat4 {
        let v: Vec3 = camera.y - (camera.y.dot(&nn) * nn);
        let vn: Vec3 = v.normalize();
        let un: Vec3 = vn.cross(&nn);

        let m14 = -camera.vrp.dot(&un);
        let m24 = -camera.vrp.dot(&vn);
        let m34 = -camera.vrp.dot(&nn);

        Mat4::new(
            un[0], un[1], un[2], m14,
            vn[0], vn[1], vn[2], m24,
            nn[0], nn[1], nn[2], m34,
            0.0, 0.0, 0.0, 1.0,
        )
    }

    #[inline(always)]
    fn calc_pers_matrix(camera: &Camera, m_sru_src: &Mat4, nn: &Vec3) -> Mat4 {
        let vp: Vec3 = camera.vrp + (camera.dp * (-nn));
        let src_vp: Mat4x1 = m_sru_src * vec3_to_mat4x1(&vp);
        let src_prp: Mat4x1 = m_sru_src * vec3_to_mat4x1(&camera.vrp);

        let m33 = -src_vp[2] / camera.dp;
        let m34 = src_vp[2] * (src_prp[2] / camera.dp);
        let m43 = -1.0 / camera.dp;
        let m44 = src_prp[2] / camera.dp;

        Mat4::new(
            1.0, 0.0, 0.0, 0.0,
            0.0, 1.0, 0.0, 0.0,
            0.0, 0.0, m33, m34,
            0.0, 0.0, m43, m44,
        )
    }

    #[inline(always)]
    fn calc_orth_matrix(&self) -> Mat4 {
        Mat4::identity()
    }

    #[inline(always)]
    fn calc_jp_matrix(window: &Window, viewport: &Viewport) -> Mat4 {
        let m11 = (viewport.umax - viewport.umin) / (window.xmax - window.xmin);
        let m14 = -window.xmin * m11 + viewport.umin;
        let m22 = (viewport.vmin - viewport.vmax) / (window.ymax - window.ymin);
        let m24 = window.ymin * ((viewport.vmax - viewport.vmin) / (window.ymax - window.ymin)) + viewport.vmax;

        Mat4::new(
            m11, 0.0, 0.0, m14,
            0.0, m22, 0.0, m24,
            0.0, 0.0, 1.0, 0.0,
            0.0, 0.0, 0.0, 1.0,
        )
    }

    #[inline(always)]
    fn calc_sru_srt_pers_matrix(camera: &Camera, window: &Window, viewport: &Viewport) -> Mat4 {
        let n: Vec3 = camera.vrp - camera.p;
        let nn: Vec3 = n.normalize();

        let m_sru_src: Mat4 = Render::calc_sru_src_matrix(&camera, &nn);
        let m_pers: Mat4 = Render::calc_pers_matrix(&camera, &m_sru_src, &nn);
        let m_jp: Mat4 = Render::calc_jp_matrix(&window, &viewport);
        let m_sru_srt: Mat4 = m_jp * (m_pers * m_sru_src);

        m_sru_srt
    }

    #[inline(always)]
    fn calc_sru_srt_orth_matrix(camera: &Camera, window: &Window, viewport: &Viewport) -> Mat4 {
        let n: Vec3 = camera.vrp - camera.p;
        let nn: Vec3 = n.normalize();

        let m_sru_src: Mat4 = Render::calc_sru_src_matrix(&camera, &nn);
        let m_pers: Mat4 = Render::calc_pers_matrix(&camera, &m_sru_src, &nn);
        let m_jp: Mat4 = Render::calc_jp_matrix(&window, &viewport);
        let m_sru_srt: Mat4 = m_jp * (m_pers * m_sru_src);

        m_sru_srt
    }

    /// Filtra os vértices que não são vizíveis através do vetor normal das faces.
    fn apply_visibility_filter(&self, vertices: &[Mat4x1], faces: &[[usize; 4]], camera: &Camera) -> Vec<[usize; 4]> {
        faces
            .iter()
            .filter_map(|face| {
                let a: Vec3 = mat4x1_to_vec3(&vertices[face[0]]);
                let b: Vec3 = mat4x1_to_vec3(&vertices[face[1]]);
                let c: Vec3 = mat4x1_to_vec3(&vertices[face[2]]);

                let bc: Vec3 = c - b;
                let ba: Vec3 = a - b;
                let nn: Vec3 = bc.cross(&ba).normalize();

                let cent: Vec3 = (a + b + c) / 3.0;
                let on: Vec3 = (camera.vrp - cent).normalize();

                if nn.dot(&on) > 0.0 {
                    Some(*face)
                } else {
                    None
                }
            })
            .collect()
    }

    /// Aplica as transformações de SRU para SRT.
    fn apply_screen_transforns(&self, vertices: &[Mat4x1]) -> Vec<Mat4x1> {
        vertices
            .iter()
            .map(|vertex| {
                let mut vertex_srt = self.m_sru_srt * vertex;
                vertex_srt[0] = (vertex_srt[0] / vertex_srt[3]).round();
                vertex_srt[1] = (vertex_srt[1] / vertex_srt[3]).round();
                vertex_srt
            })
            .collect()
    }

    /// Calcula as interseções das arestas da face com as linhas horizontais.
    pub fn calculate_intersections(
        vertices: &[Mat4x1],
        face: &[usize; 4],
    ) -> BTreeMap<usize, Vec<OrderedFloat<f32>>> {
        let mut intersections: BTreeMap<usize, Vec<OrderedFloat<f32>>> = BTreeMap::new();

        for i in 0..3 {
            let mut x0 = vertices[face[i]].x;
            let mut y0 = vertices[face[i]].y.round();
            let mut x1 = vertices[face[i + 1]].x;
            let mut y1 = vertices[face[i + 1]].y.round();

            if y0 > y1 {
                let x = x0;
                x0 = x1;
                x1 = x;

                let y = y0;
                y0 = y1;
                y1 = y;
            }

            let dx = x1 - x0;
            let dy = y1 - y0;
            let tx = dx / dy;

            let mut x = x0;
            let mut y = y0.round();

            while y < y1 {
                if y >= 0.0 {
                    let x_intersections = intersections.entry(y as usize)
                        .or_insert(Vec::new());
                    x_intersections.push(x.into());
                }
                x += tx;
                y += 1.0;
            }
        }

        for (_, intersections) in intersections.iter_mut() {
            intersections.sort();
        }

        intersections
    }

    pub fn render(
        &self,
        vertices: &[Mat4x1],
        faces: &[[usize; 4]],
        camera: &Camera,
    ) {
        let visible_faces: Vec<[usize; 4]> = self.apply_visibility_filter(vertices, faces, camera);
        let transformed_vertices: Vec<Mat4x1> = self.apply_screen_transforns(vertices);

        visible_faces
            .iter()
            .for_each(|face| {
                // Para cada face, calcula as interseções da varredura
                for (i, x_intersections) in Render::calculate_intersections(&transformed_vertices, face) {
                    let mut counter = 0;

                    while counter < x_intersections.len() {
                        let x_initial = x_intersections[counter].ceil() as usize;
                        let x_final   = x_intersections[counter + 1].floor() as usize;

                        // Desenho (ou pintura) linha a linha
                        for j in x_initial..=x_final {
                            match self.shader_type {
                                ShaderType::Wireframe => {
                                    // Desenha a linha horizontal
                                }
                                ShaderType::Constant => {
                                    // Pinta a face com cor constante
                                }
                                ShaderType::Gouraud => {
                                    // Pinta a face com a interpolação de cores dos vértices
                                }
                                ShaderType::Phong => {
                                    // Pinta a face com a interpolação de cores dos vértices
                                }
                            }
                        }

                        counter += 2;
                    }
                }
            });
    }
}