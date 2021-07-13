use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

use crate::webgl; 

use web_sys::{
    WebGl2RenderingContext
};
pub struct GLHandler {
    context: WebGl2RenderingContext
}

impl GLHandler {
    pub fn new(&self, canvas: &web_sys::HtmlCanvasElement) -> Result<GLHandler, JsValue> {
        let context = canvas
            .get_context("webgl2")?
            .unwrap()
            .dyn_into::<WebGl2RenderingContext>()?;
        return Ok(
            GLHandler {
                context,
            }
        )
    }
    pub fn use_program(&self, vert_url: &str, frag_url: &str) -> Result<(), JsValue>{
        let vert_shader = webgl::shader_from_url(
            &self.context, 
            WebGl2RenderingContext::VERTEX_SHADER,
            vert_url
        )?;
    
        let frag_shader = webgl::shader_from_url(
            &self.context, 
            WebGl2RenderingContext::FRAGMENT_SHADER,
            frag_url
        )?;

        let program = webgl::link_program(&self.context, &vert_shader, &frag_shader)?;
        self.context.use_program(Some(&program));
        Ok(())
    }
}