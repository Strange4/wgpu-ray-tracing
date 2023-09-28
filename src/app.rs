use eframe::{egui, glow, Frame};
use std::sync::{Arc, Mutex};
use std::time::Instant;

pub struct AppUI {
    name: String,
    angle: f32,
    ray_tracer: Arc<Mutex<RayTracer>>,
}

impl AppUI {
    pub fn new(eframe_context: &eframe::CreationContext) -> Self {
        let glow_context = eframe_context
            .gl
            .as_ref()
            .expect("There should be a glow context here");
        return AppUI {
            name: "Hello".into(),
            angle: 0.0,
            ray_tracer: Arc::new(Mutex::new(RayTracer::new(
                &glow_context,
                eframe_context.egui_ctx.clone(),
            ))),
        };
    }

    fn ray_tracer_ui(&mut self, ui: &mut egui::Ui) {
        let ray_tracer = self.ray_tracer.clone();
        {
            ui.label(format!("{}ns", ray_tracer.lock().unwrap().last_frame_time));
        }
        let (canvas, response) = ui.allocate_exact_size(ui.available_size(), egui::Sense::drag());
        self.angle += response.drag_delta().x * 0.01;
        let angle = self.angle;
        let callback = egui::PaintCallback {
            rect: canvas,
            callback: Arc::new(eframe::egui_glow::CallbackFn::new(move |_info, painter| {
                let start = Instant::now();
                let mut ray_tracer = ray_tracer
                    .lock()
                    .expect("Bro I couldn't get the ray tracer idk");
                ray_tracer.render(painter.gl(), angle);
                // for i in 0..100_000_000 {}
                let end = Instant::now();
                let time = end - start;
                ray_tracer.last_frame_time = time.as_nanos();
                ray_tracer.egui_context.lock().unwrap().request_repaint();
            })),
        };
        ui.painter().add(callback);
    }
}

impl eframe::App for AppUI {
    fn update(&mut self, context: &egui::Context, _frame: &mut Frame) {
        egui::CentralPanel::default().show(context, |ui| {
            ui.heading(self.name.clone());
            ui.horizontal(|ui| {
                ui.label("App name here: ");
                ui.text_edit_singleline(&mut self.name);
            });

            ui.label(format!("Drag it '{}'!", self.name));
            egui::Frame::canvas(ui.style()).show(ui, |ui| {
                self.ray_tracer_ui(ui);
            });
        });
    }

    fn on_exit(&mut self, gl: Option<&glow::Context>) {
        if let Some(gl) = gl {
            self.ray_tracer
                .lock()
                .expect("i am under the waterx")
                .destroy(gl);
        }
    }
}

pub struct RayTracer {
    program: eframe::glow::Program,
    vertex_array: eframe::glow::VertexArray,
    last_frame_time: u128,
    egui_context: Arc<Mutex<egui::Context>>,
}

impl RayTracer {
    pub fn new(gl: &eframe::glow::Context, egui_context: egui::Context) -> Self {
        use glow::HasContext as _;

        let shader_version = if cfg!(target_arch = "wasm32") {
            "#version 300 es"
        } else {
            "#version 330"
        };

        unsafe {
            let program = gl.create_program().expect("Cannot create program");
            let (vertex_shader_source, fragment_shader_source) = (
                r#"
                    const vec2 verts[3] = vec2[3](
                        vec2(0.0, 1.0),
                        vec2(-1.0, -1.0),
                        vec2(1.0, -1.0)
                    );
                    const vec4 colors[3] = vec4[3](
                        vec4(1.0, 0.0, 0.0, 1.0),
                        vec4(0.0, 1.0, 0.0, 1.0),
                        vec4(0.0, 0.0, 1.0, 1.0)
                    );
                    out vec4 v_color;
                    uniform float u_angle;
                    void main() {
                        v_color = colors[gl_VertexID];
                        gl_Position = vec4(verts[gl_VertexID], 0.0, 1.0);
                        gl_Position.x *= cos(u_angle);
                    }
                "#,
                r#"
                    precision mediump float;
                    in vec4 v_color;
                    out vec4 out_color;
                    void main() {
                        out_color = v_color;
                    }
                "#,
            );

            let shader_sources = [
                (glow::VERTEX_SHADER, vertex_shader_source),
                (glow::FRAGMENT_SHADER, fragment_shader_source),
            ];

            let shaders: Vec<_> = shader_sources
                .iter()
                .map(|(shader_type, shader_source)| {
                    let shader = gl
                        .create_shader(*shader_type)
                        .expect("Cannot create shader");
                    gl.shader_source(shader, &format!("{shader_version}\n{shader_source}"));
                    gl.compile_shader(shader);
                    assert!(
                        gl.get_shader_compile_status(shader),
                        "Failed to compile {shader_type}: {}",
                        gl.get_shader_info_log(shader)
                    );
                    gl.attach_shader(program, shader);
                    shader
                })
                .collect();

            gl.link_program(program);
            assert!(
                gl.get_program_link_status(program),
                "Failed to link to the opengl program{}",
                gl.get_program_info_log(program)
            );

            for shader in shaders {
                gl.detach_shader(program, shader);
                gl.delete_shader(shader);
            }

            let vertex_array = gl
                .create_vertex_array()
                .expect("Cannot create vertex array");

            Self {
                program,
                vertex_array,
                last_frame_time: 0,
                egui_context: Arc::new(Mutex::new(egui_context)),
            }
        }
    }
    fn render(&self, gl: &glow::Context, angle: f32) {
        use glow::HasContext as _;
        unsafe {
            gl.use_program(Some(self.program));
            gl.uniform_1_f32(
                gl.get_uniform_location(self.program, "u_angle").as_ref(),
                angle,
            );
            gl.bind_vertex_array(Some(self.vertex_array));
            gl.draw_arrays(glow::TRIANGLES, 0, 3);
        }
    }

    fn destroy(&self, gl: &glow::Context) {
        use glow::HasContext as _;
        unsafe {
            gl.delete_program(self.program);
            gl.delete_vertex_array(self.vertex_array);
        }
    }
}
