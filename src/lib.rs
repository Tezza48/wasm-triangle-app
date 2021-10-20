mod utils;
mod sprite;
mod asset;
mod gl;

use sprite::{Sprite, SpriteRenderer};
use wasm_bindgen::{JsCast, prelude::*};
use winit::{event::{Event, WindowEvent}, event_loop::{ControlFlow, EventLoop}, window::WindowBuilder};

use crate::asset::{Asset, Texture};

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen(module = "/src/stats.jsShim.js")]
extern "C" {
    type Stats;

    #[wasm_bindgen(constructor)]
    fn new() -> Stats;

    #[wasm_bindgen(method, getter)]
    fn dom(this: &Stats) -> web_sys::HtmlElement;

    #[wasm_bindgen(method)]
    fn update(this: &Stats);
}

#[wasm_bindgen(start)]
pub fn start() {
    let event_loop = EventLoop::new();

    let window = WindowBuilder::new()
        .build(&event_loop)
        .expect("Failed to create window.");

    use winit::platform::web::WindowExtWebSys;

    let canvas = window
        .canvas();

    canvas.set_id("game-canvas");

    window.canvas().dyn_into::<web_sys::HtmlElement>().unwrap()
        .style().set_css_text("
            top: 50%;
            left: 50%;
        ");


    let stats = Stats::new();

    {
        let window = web_sys::window().expect("Could not get Window from browser");
        let document = window.document().expect("Could not get document from window");
        let body = document.body().expect("Could not get body from document");

        body.append_child(&stats.dom()).unwrap();
        body.append_child(&canvas).expect("Could not append canvas to body");

        let body_style = body.dyn_into::<web_sys::HtmlElement>().unwrap().style();
        body_style.set_property("padding", "0").unwrap();
        body_style.set_property("margin", "0").unwrap();
    }
    use web_sys::WebGl2RenderingContext as GL;

    let gl = canvas
        .get_context("webgl2")
        .expect("Canvas webgl2 context")
        .expect("Canvas could not produce webgl2 context")
        .dyn_into::<web_sys::WebGl2RenderingContext>()
        .expect("Could not convert context to WebGl2RenderingContext");



    gl.clear_color(0.1, 0.1, 0.1, 1.0);

    let sprite_renderer = SpriteRenderer::new(&gl);

    let mut assets = asset::AssetCache::new();
    assets.load_texture(&gl, String::from("https://images.unsplash.com/photo-1634549709262-508c47d4c229?ixlib=rb-1.2.1&ixid=MnwxMjA3fDB8MHxwaG90by1wYWdlfHx8fGVufDB8fHx8&auto=format&fit=crop&w=687&q=80"));

    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Poll;

        match event {
            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                window_id,
            } if window_id == window.id() => {
                *control_flow = ControlFlow::Exit;

                sprite_renderer.destroy(&gl);
            },
            Event::MainEventsCleared => {
                window.request_redraw();
            },
            Event::RedrawRequested(_) => {
                gl.clear(GL::COLOR_BUFFER_BIT);

                sprite_renderer.render_sprites(&gl, &assets, &[Sprite {
                    texture: Asset::new(0),
                }]);

                stats.update();
            },
            _ => (),
        }
    })
}