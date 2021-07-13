use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

use web_sys::{
    Element, 
    WebGlProgram, 
    WebGl2RenderingContext, 
    WebGlShader, 
    WebGlVertexArrayObject,
    window
};

pub mod webgl;

#[wasm_bindgen(start)]
pub fn start() -> Result<(), JsValue> {
    let document = web_sys::window()
        .unwrap()
        .document()
        .unwrap();
    let canvas = document
        .get_element_by_id("canvas")
        .unwrap();

    let canvas: web_sys::HtmlCanvasElement = canvas.dyn_into::<web_sys::HtmlCanvasElement>()?;
    let window = window().ok_or("Failed to access Window API")?; 

    let inner_height = window
        .inner_height()?
        .as_f64()
        .unwrap() as i32; 

    let inner_width = window
        .inner_width()?
        .as_f64()
        .unwrap() as i32; 

    canvas.set_width(inner_width as u32);
    canvas.set_height(inner_height as u32);

    let context = canvas
        .get_context("webgl2")?
        .unwrap()
        .dyn_into::<WebGl2RenderingContext>()?;
    
    context.viewport(0, 0, inner_width, inner_height);

    let vert_shader = webgl::shader_from_url(
        &context, 
        WebGl2RenderingContext::VERTEX_SHADER,
        "../shaders/vert_shader.glsl"
    )?;

    let frag_shader = webgl::shader_from_url(
        &context, 
        WebGl2RenderingContext::FRAGMENT_SHADER,
        "../shaders/frag_shader.glsl"
    )?;

    

    let program = webgl::link_program(&context, &vert_shader, &frag_shader)?;
    context.use_program(Some(&program));

    let vertices: [f32; 9] = [-0.7, -0.7, 0.0, 0.7, -0.7, 0.0, 0.0, 0.7, 0.0];

    let position_attribute_location = context.get_attrib_location(&program, "position");
    let buffer = context.create_buffer().ok_or("Failed to create buffer")?;
    context.bind_buffer(WebGl2RenderingContext::ARRAY_BUFFER, Some(&buffer));

    unsafe {
        let positions_array_buf_view = js_sys::Float32Array::view(&vertices);

        context.buffer_data_with_array_buffer_view(
            WebGl2RenderingContext::ARRAY_BUFFER,
            &positions_array_buf_view,
            WebGl2RenderingContext::STATIC_DRAW,
        );
    }

    let vao = context
        .create_vertex_array()
        .ok_or("Could not create vertex array object")?;

    context.bind_vertex_array(Some(&vao));

    context.vertex_attrib_pointer_with_i32(0, 3, WebGl2RenderingContext::FLOAT, false, 0, 0);
    context.enable_vertex_attrib_array(position_attribute_location as u32);

    context.bind_vertex_array(Some(&vao));

    let vert_count = (vertices.len() / 3) as i32;
    webgl::draw(&context, vert_count);

    Ok(())

}
 