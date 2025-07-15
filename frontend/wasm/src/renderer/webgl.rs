use crate::utils::webgl_utils::{compile_shader, link_program};
use std::cell::RefCell;
use std::rc::Rc;
use wasm_bindgen::JsCast;
use wasm_bindgen::{JsValue, closure::Closure};
use web_sys::{
    HtmlCanvasElement, WebGl2RenderingContext, WebGlUniformLocation, WebGlVertexArrayObject,
};

pub struct State {
    gl: WebGl2RenderingContext,
    vao: WebGlVertexArrayObject,
    u_resolution: WebGlUniformLocation,
    u_time: WebGlUniformLocation,
    start_time_ms: f64,
    frame_count: u64,
    last_fps_update: f64,
    fps: f64,
    frame_times: Vec<f64>,
}
pub fn render_webgl_canvas(canvas_id: &str, vert_src: &str, frag_src: &str) -> Result<(), JsValue> {
    let window = web_sys::window().ok_or("no window")?;
    let document = window.document().ok_or("no document")?;
    let canvas: HtmlCanvasElement = document
        .get_element_by_id(canvas_id)
        .ok_or("#canvas not found")?
        .dyn_into()?;

    let gl: WebGl2RenderingContext = canvas
        .get_context("webgl2")?
        .ok_or("WebGL2 not available")?
        .dyn_into()?;

    // ── SHADERS & PROGRAM ─────────────────────────────────────────────────────
    let vert_shader = compile_shader(&gl, WebGl2RenderingContext::VERTEX_SHADER, vert_src)?;
    let frag_shader = compile_shader(&gl, WebGl2RenderingContext::FRAGMENT_SHADER, frag_src)?;
    let program = link_program(&gl, &vert_shader, &frag_shader)?;
    gl.use_program(Some(&program));
    let vertices: [f32; 12] = [
        -1.0, -1.0, // ⌜ left‑bottom
        1.0, -1.0, // ⌝ right‑bottom
        -1.0, 1.0, // ⌟ left‑top
        -1.0, 1.0, // ⌝
        1.0, -1.0, // ⌞
        1.0, 1.0, // ⌟ right‑top
    ];

    let vao = gl.create_vertex_array().ok_or("cannot create VAO")?;
    gl.bind_vertex_array(Some(&vao));

    let vbo = gl.create_buffer().ok_or("cannot create VBO")?;
    gl.bind_buffer(WebGl2RenderingContext::ARRAY_BUFFER, Some(&vbo));
    unsafe {
        let vert_view = js_sys::Float32Array::view(&vertices);
        gl.buffer_data_with_array_buffer_view(
            WebGl2RenderingContext::ARRAY_BUFFER,
            &vert_view,
            WebGl2RenderingContext::STATIC_DRAW,
        );
    }

    let pos_loc = gl.get_attrib_location(&program, "a_position") as u32;
    gl.enable_vertex_attrib_array(pos_loc);
    gl.vertex_attrib_pointer_with_i32(pos_loc, 2, WebGl2RenderingContext::FLOAT, false, 0, 0);

    // ── UNIFORMS ───────────────────────────────────────────────────────────────
    let u_resolution = gl
        .get_uniform_location(&program, "iResolution")
        .ok_or("iResolution uniform not found")?;
    let u_time = gl
        .get_uniform_location(&program, "iTime")
        .ok_or("iTime uniform not found")?;

    let start_time_ms = window.performance().unwrap().now();

    let state = Rc::new(RefCell::new(State {
        gl,
        vao,
        u_resolution,
        u_time,
        start_time_ms,
        frame_count: 0,
        last_fps_update: start_time_ms,
        fps: 0.0,
        frame_times: Vec::with_capacity(60), // Store last 60 frame times
    }));

    // ── RENDER LOOP WITH FRAME TRACKING ────────────────────────────────────────
    let g_state = state.clone();
    let f: Rc<RefCell<Option<Closure<dyn FnMut(f64)>>>> = Rc::new(RefCell::new(None));
    let g = f.clone();
    let width = 1000;
    let height = 1000;

    *g.borrow_mut() = Some(Closure::wrap(Box::new(move |timestamp: f64| {
        let fps_counter_element = document.get_element_by_id("fps-counter-element");
        let mut s = g_state.borrow_mut();

        // ── FRAME TRACKING ─────────────────────────────────────────────────────
        s.frame_count += 1;

        // Store frame time for averaging
        s.frame_times.push(timestamp);
        if s.frame_times.len() > 60 {
            s.frame_times.remove(0);
        }

        // Calculate FPS every second
        if timestamp - s.last_fps_update >= 1000.0 {
            s.fps = s.frame_count as f64 / ((timestamp - s.start_time_ms) / 1000.0);
            s.last_fps_update = timestamp;

            // Calculate average frame time from last 60 frames
            if s.frame_times.len() >= 2 {
                fps_counter_element
                    .unwrap()
                    .set_inner_html(&format!("Current FPS: {:.1}", s.fps));
            }
        }

        // ── RENDERING ───────────────────────────────────────────────────────────
        // Time uniform (seconds since start)
        let t = ((timestamp - s.start_time_ms) / 1000.0) as f32;
        s.gl.uniform1f(Some(&s.u_time), t);

        // Resolution uniform (updated in case the canvas was resized)
        s.gl.uniform3f(Some(&s.u_resolution), width as f32, height as f32, 0.0);

        // Rendering
        s.gl.viewport(0, 0, width as i32, height as i32);
        s.gl.clear_color(1.0, 1.0, 1.0, 0.0);
        s.gl.clear(WebGl2RenderingContext::COLOR_BUFFER_BIT);
        s.gl.bind_vertex_array(Some(&s.vao));
        s.gl.draw_arrays(WebGl2RenderingContext::TRIANGLES, 0, 6);

        // Queue next frame
        web_sys::window()
            .unwrap()
            .request_animation_frame(f.borrow().as_ref().unwrap().as_ref().unchecked_ref())
            .unwrap();
    }) as Box<dyn FnMut(f64)>));
    canvas.set_width(width);
    canvas.set_height(height);
    window.request_animation_frame(g.borrow().as_ref().unwrap().as_ref().unchecked_ref())?;

    Ok(())
}
