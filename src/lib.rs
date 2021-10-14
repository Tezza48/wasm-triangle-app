mod utils;

use std::mem::size_of;

use wasm_bindgen::{JsCast, prelude::*};
use winit::{event::{Event, WindowEvent}, event_loop::{ControlFlow, EventLoop}, window::WindowBuilder};

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen(start)]
pub fn start() {
    let event_loop = EventLoop::new();

    let window = WindowBuilder::new()
        .build(&event_loop)
        .expect("Failed to create window.");

    use winit::platform::web::WindowExtWebSys;

    let canvas = window
        .canvas()
        .dyn_into::<web_sys::HtmlCanvasElement>()
        .expect("Could not convert canvas from Node to HtmlCanvasElement");

    canvas.set_id("game-canvas");

    window.canvas().dyn_into::<web_sys::HtmlElement>().unwrap()
        .style().set_css_text("
            top: 50%;
            left: 50%;
        ");

    {
        let window = web_sys::window().expect("Could not get Window from browser");
        let document = window.document().expect("Could not get document from window");
        let body = document.body().expect("Could not get body from document");

        body.append_child(&canvas).expect("Could not append canvas to body");
    }

    use web_sys::WebGl2RenderingContext as GL;

    let gl = canvas
        .get_context("webgl2")
        .expect("Canvas webgl2 context")
        .expect("Canvas could not produce webgl2 context")
        .dyn_into::<web_sys::WebGl2RenderingContext>()
        .expect("Could not convert context to WebGl2RenderingContext");

    let vao = gl.create_vertex_array().unwrap();
    let vbo = gl.create_buffer().unwrap();
    let verts: &[f32] = &[
        0.0, 0.5,
        -0.5, -0.5,
        0.5, -0.5
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

    gl.clear_color(0.1, 0.1, 0.1, 1.0);

    let program = gl.create_program().unwrap();
    let v_shader = gl.create_shader(GL::VERTEX_SHADER).unwrap();
    let f_shader = gl.create_shader(GL::FRAGMENT_SHADER).unwrap();

    gl.shader_source(
        &v_shader,
        "#version 300 es

        in vec2 vert_pos;

        void main() {
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

        precision highp float;

        out vec4 color;

        void main() {
            color = vec4(1.0);
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

    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Poll;

        match event {
            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                window_id,
            } if window_id == window.id() => *control_flow = ControlFlow::Exit,
            Event::MainEventsCleared => {
                window.request_redraw();
            },
            Event::RedrawRequested(_) => {
                gl.clear(GL::COLOR_BUFFER_BIT);
                gl.bind_vertex_array(Some(&vao));

                gl.use_program(Some(&program));

                gl.draw_arrays(GL::TRIANGLES, 0, 3);
            },
            _ => (),
        }
    })
}