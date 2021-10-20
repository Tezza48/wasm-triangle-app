use std::ops::Div;

use web_sys::{WebGlBuffer, WebGlVertexArrayObject};

use crate::gl::GL;

use crate::asset::*;

pub struct Sprite {
    pub texture: Asset<Texture>,
    // x: f32,
    // y: f32,
    // width: f32,
    // height: f32,
}

pub struct SpriteRenderer {
    program: web_sys::WebGlProgram,
    vao: WebGlVertexArrayObject,
    vbo: WebGlBuffer,
}

impl SpriteRenderer {
    pub fn new(gl: &GL) -> SpriteRenderer {
        let program = gl.create_program().unwrap();
        let v_shader = gl.create_shader(GL::VERTEX_SHADER).unwrap();
        let f_shader = gl.create_shader(GL::FRAGMENT_SHADER).unwrap();

        gl.shader_source(
            &v_shader,
            "#version 300 es

            in vec2 vert_pos;

            out vec2 pos;

            void main() {
                pos = vert_pos;
                gl_Position = vec4(vert_pos, 0.0, 1.0);
            }
        ");

        gl.compile_shader(&v_shader);
        if let Some(log) = gl.get_shader_info_log(&v_shader) {
            if log.len() != 0 {
                web_sys::console::log_1(
                    &wasm_bindgen::JsValue::from_str(
                        format!("Error compiling v shader: {}", &log)
                        .as_str()
                    )
                );
            }
        }

        gl.shader_source(
            &f_shader,
            "#version 300 es

            uniform sampler2D tex;

            precision highp float;

            in vec2 pos;

            out vec4 color;

            void main() {
                color = vec4(texture(tex, vec2(pos.x, 1.0 - pos.y)).rgb, 1.0);
            }
        ");

        gl.compile_shader(&f_shader);
        if let Some(log) = gl.get_shader_info_log(&f_shader) {
            if log.len() != 0 {
                web_sys::console::log_1(
                    &wasm_bindgen::JsValue::from_str(
                        format!("Error compiling f shader: {}", &log)
                        .as_str()
                    )
                );
            }
        }

        gl.attach_shader(&program, &v_shader);
        gl.attach_shader(&program, &f_shader);
        gl.link_program(&program);

        let vao = gl.create_vertex_array().unwrap();
        let vbo = gl.create_buffer().unwrap();
        let verts: &[f32] = &[
            0.0, 0.0,
            0.0, 1.0,
            1.0, 1.0,
            0.0, 0.0,
            1.0, 1.0,
            1.0, 0.0
        ];
        gl.bind_vertex_array(Some(&vao));
        gl.bind_buffer(GL::ARRAY_BUFFER, Some(&vbo));
        gl.buffer_data_with_u8_array(
            GL::ARRAY_BUFFER,
            bytemuck::cast_slice(verts),
            GL::STATIC_DRAW
        );

        gl.enable_vertex_attrib_array(0);
        gl.vertex_attrib_pointer_with_i32(0, 2, GL::FLOAT, false, 0, 0);

        SpriteRenderer {
            program,
            vao,
            vbo,
        }
    }
    pub fn render_sprites(&self, gl: &GL, assets: &crate::asset::AssetCache, sprites: &[Sprite]) {
        gl.use_program(Some(&self.program));
        gl.bind_vertex_array(Some(&self.vao));


        sprites.iter().for_each(|sprite| {
            gl.active_texture(GL::TEXTURE0);
            gl.bind_texture(GL::TEXTURE_2D, Some(&assets.get_texture(&sprite.texture).texture));
            gl.draw_arrays(GL::TRIANGLES, 0, 6);
        });
    }

    pub fn destroy(&self, gl: &GL) {
        gl.delete_program(Some(&self.program));
        gl.delete_vertex_array(Some(&self.vao));
        gl.delete_buffer(Some(&self.vbo));
    }
}