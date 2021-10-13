mod utils;

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

    let context = canvas
        .get_context("2d")
        .expect("Canvas 2d context")
        .expect("Canvas could not produce 2d context")
        .dyn_into::<web_sys::CanvasRenderingContext2d>()
        .expect("Could not convert context to HtmlCanvasRenderingContext2D");

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
            Event::RedrawRequested(window_id) => {
                context.set_fill_style(&JsValue::from_str("aqua"));
                context.fill_rect(0.0, 0.0, canvas.width() as f64, canvas.height() as f64);
            },
            _ => (),
        }
    })
}