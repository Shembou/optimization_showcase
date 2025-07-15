use wasm_bindgen::{JsValue, prelude::wasm_bindgen};

use crate::{
    renderer::webgl::render_webgl_canvas,
    shaders::{
        accretion::{ACCCRETION_VERT_SRC, ACCRETION_FRAG_SRC},
        raymarching::{RAYMARCHING_FRAG_SRC, RAYMARCHING_VERT_SRC},
    }, utils::canvas_utils::draw_julia_canvas,
};

pub mod renderer;
pub mod shaders;
pub mod utils;
#[wasm_bindgen]
pub fn render_accretion(canvas_id: &str) -> Result<(), JsValue> {
    return render_webgl_canvas(canvas_id, ACCCRETION_VERT_SRC, ACCRETION_FRAG_SRC);
}

#[wasm_bindgen]
pub fn render_raymarching(canvas_id: &str) -> Result<(), JsValue> {
    return render_webgl_canvas(canvas_id, RAYMARCHING_VERT_SRC, RAYMARCHING_FRAG_SRC);
}

#[wasm_bindgen]
pub fn draw_julia_set(cavnas_id: &str, width: u32, height: u32, real: f64, imaginary: f64) -> Result<(), JsValue> {
    return draw_julia_canvas(cavnas_id,width,height,real,imaginary);
}
