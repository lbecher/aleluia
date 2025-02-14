use rand::Rng;
use std::sync::{Arc, Mutex};
use crate::types::Mat4x1;

/// Estrutura para armazenar uma superfície BSpline.
#[derive(Debug)]
pub struct Object {
    /// Quantidades de pontos de controle na direção i.
    ni: usize,
    /// Quantidades de pontos de controle na direção j.
    nj: usize,
    /// Ordem da spline (grau do polinômio interpolador) na direção i.
    ti: usize,
    /// Ordem da spline (grau do polinômio interpolador) na direção j.
    tj: usize,
    /// Resolução na direção i.
    resi: usize,
    /// Resolução na direção j.
    resj: usize,

    /// Nós (knots) na direção i.
    knots_i: Vec<f32>,
    /// Nós (knots) na direção j.
    knots_j: Vec<f32>,

    /// Pontos de controle.
    pub control_points: Vec<Mat4x1>,

    /// Lista de vertices da malha interpolada.
    vertices: Vec<Mat4x1>,
    /// Lista de faces da malha interpolada.
    faces: Vec<[usize; 4]>,
}

impl Object {
    pub fn new(
        ni: usize,
        nj: usize,
        ti: usize,
        tj: usize,
        resi: usize,
        resj: usize,
    ) -> Self {
        let mut rng = rand::thread_rng();

        let mut control_points: Vec<Mat4x1> = Vec::with_capacity((ni + 1) * (nj + 1));
        for i in 0..=ni {
            for j in 0..=nj {
                control_points.push(Mat4x1::new(
                    i as f32,
                    j as f32,
                    rng.gen_range(0.0..10.0),
                    1.0,
                ));
            }
        }

        let knots_i: Vec<f32> = Self::spline_knots(ni, ti);
        let knots_j: Vec<f32> = Self::spline_knots(nj, tj);

        let mut obj = Self {
            ni,
            nj,
            ti,
            tj,
            resi,
            resj,
            control_points,

            knots_i,
            knots_j,

            vertices: vec![Mat4x1::zeros(); resi * resj],
            faces: Vec::with_capacity((resi - 1) * (resj - 1)),
        };

        obj.gen_mesh();

        obj
    }

    /// Gera a malha da superfície.
    pub fn gen_mesh(&mut self) {
        // Zera os vértices iniciais
        for ipt in &mut self.vertices {
            *ipt = Mat4x1::new(0.0, 0.0, 0.0, 1.0);
        }

        // Cálculo dos incrementos
        let increment_i = (self.ni as f32 - self.ti as f32 + 2.0) / self.resi as f32;
        let increment_j = (self.nj as f32 - self.tj as f32 + 2.0) / self.resj as f32;

        // Vamos definir um número de threads (por exemplo, 4)
        let n_threads = 4;

        // Dividimos as linhas (i) em blocos
        let chunk_size = (self.resi + n_threads - 1) / n_threads;

        // Para facilitar o acesso concorrente, usamos Arc para ler e escrever de forma segura
        // - knots_i e knots_j, control_points são apenas lidos (podem ser compartilhados sem Mutex).
        // - vertices precisa de Mutex para escrita simultânea.
        let arc_knots_i = Arc::new(self.knots_i.clone());
        let arc_knots_j = Arc::new(self.knots_j.clone());
        let arc_control_points = Arc::new(self.control_points.clone());

        // Precisamos de Mutex para poder escrever em 'vertices' paralelamente
        let arc_vertices = Arc::new(Mutex::new(vec![Mat4x1::zeros(); self.resi * self.resj]));

        // Clonamos valores necessários (por simplicidade, eles podem ser copiados ou clonados)
        let ni = self.ni;
        let nj = self.nj;
        let ti = self.ti;
        let tj = self.tj;
        let resi = self.resi;
        let resj = self.resj;

        // Vetor de handles de thread
        let mut handles = Vec::with_capacity(n_threads);

        for t in 0..n_threads {
            // Definimos o range de i para cada thread
            let start_i = t * chunk_size;
            let end_i = (start_i + chunk_size).min(resi);

            // Clonamos as referências compartilhadas
            let knots_i = Arc::clone(&arc_knots_i);
            let knots_j = Arc::clone(&arc_knots_j);
            let control_points = Arc::clone(&arc_control_points);
            let vertices = Arc::clone(&arc_vertices);

            // Spawn da thread
            let handle = std::thread::spawn(move || {
                // Vetor local para armazenar o resultado parcial
                let mut local_vertices =
                    vec![Mat4x1::new(0.0, 0.0, 0.0, 1.0); (end_i - start_i) * resj];

                // Iniciamos o intervalo de i de acordo com start_i
                let mut interval_i = start_i as f32 * increment_i;

                for i in start_i..end_i {
                    let mut interval_j = 0.0;

                    for j in 0..resj {
                        let local_idx = (i - start_i) * resj + j;

                        // Soma as contribuições de cada ponto de controle
                        for ki in 0..=ni {
                            for kj in 0..=nj {
                                let bi = Self::spline_blend(ki, ti, &knots_i, interval_i);
                                let bj = Self::spline_blend(kj, tj, &knots_j, interval_j);

                                let blend = bi * bj;
                                let cp_idx = ki * (nj + 1) + kj;

                                local_vertices[local_idx] =
                                    local_vertices[local_idx] + control_points[cp_idx] * blend;
                            }
                        }
                        interval_j += increment_j;
                    }
                    interval_i += increment_i;
                }

                // Copiamos o resultado parcial para o vetor global
                {
                    let mut global_vertices = vertices.lock().unwrap();
                    for i_local in 0..(end_i - start_i) {
                        for j_local in 0..resj {
                            let local_out_idx = i_local * resj + j_local;
                            let global_out_idx = (start_i + i_local) * resj + j_local;
                            global_vertices[global_out_idx] = local_vertices[local_out_idx];
                        }
                    }
                }
            });
            handles.push(handle);
        }

        // Esperamos todas as threads terminarem
        for handle in handles {
            handle.join().unwrap();
        }

        // Agora podemos recuperar os vértices calculados para dentro de self.vertices
        {
            let final_vertices = arc_vertices.lock().unwrap();
            self.vertices.copy_from_slice(&final_vertices);
        }

        // Por fim, geramos as faces (pode ser paralelizado também, mas aqui está em uma thread só)
        self.faces.clear();
        for i in 0..resi - 1 {
            for j in 0..resj - 1 {
                self.faces.push([
                    i * resj + j,
                    i * resj + (j + 1),
                    (i + 1) * resj + (j + 1),
                    (i + 1) * resj + j,
                ]);
            }
        }
    }

    /// Gera o vetor de nós (knots).
    fn spline_knots(n: usize, t: usize) -> Vec<f32> {
        let mut knots = Vec::with_capacity(n + t + 1);
        for j in 0..=(n + t) {
            if j < t {
                knots.push(0.0);
            } else if j <= n {
                knots.push((j + 1 - t) as f32);
            } else {
                knots.push((n + 2 - t) as f32);
            }
        }
        knots
    }

    /// Função de base da spline recursiva.
    fn spline_blend(k: usize, t: usize, u: &[f32], v: f32) -> f32 {
        if t == 1 {
            if u[k] <= v && v < u[k + 1] {
                1.0
            } else {
                0.0
            }
        } else {
            let mut value = 0.0;
            let denom1 = u[k + t - 1] - u[k];
            let denom2 = u[k + t] - u[k + 1];

            if denom1 != 0.0 {
                value += ((v - u[k]) / denom1) * Self::spline_blend(k, t - 1, u, v);
            }
            if denom2 != 0.0 {
                value += ((u[k + t] - v) / denom2) * Self::spline_blend(k + 1, t - 1, u, v);
            }
            value
        }
    }

    /// Retorna slice imutável para vértices da malha
    pub fn get_vertices(&self) -> &[Mat4x1] {
        &self.vertices
    }

    /// Retorna slice imutável para as faces interpolados
    pub fn get_faces(&self) -> &[[usize; 4]] {
        &self.faces
    }
}
