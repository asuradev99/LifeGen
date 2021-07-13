use wasm_bindgen::prelude::*;
use web_sys::{
    WebGlProgram, 
    WebGl2RenderingContext, 
    WebGlShader, 
    XmlHttpRequest
};

pub fn f_read(url: &str) -> Result<String, JsValue> {
    let req = XmlHttpRequest::new()?;
    req.open_with_async("GET", url, false)?;
    req.send()?;
    match req.response_text() {
        Ok(response) => {
            match response {
                Some(text) => return Ok(text),
                None => return Ok("failed".to_string())
            }
        },
        Err(err) => return Err(err)
    }
}

pub fn shader_from_url(
    context: &WebGl2RenderingContext, 
    shader_type: u32, 
    url: &str
) -> Result<WebGlShader, JsValue> {
    let source = f_read(url)?;
    let shader = context
        .create_shader(shader_type)
        .ok_or_else(|| String::from("Unable to create shader object"))?;
    context.shader_source(&shader, source.as_str());
    context.compile_shader(&shader);
    if context
        .get_shader_parameter(&shader, WebGl2RenderingContext::COMPILE_STATUS)
        .as_bool()
        .unwrap_or(false)
    {
        Ok(shader)
    } else {
        Err(context
            .get_shader_info_log(&shader)
            .unwrap_or_else(|| String::from("Unknown error creating shader"))
            .into()
        )
    }
}

pub fn link_program(
    context: &WebGl2RenderingContext,
    vert_shader: &WebGlShader,
    frag_shader: &WebGlShader,
) -> Result<WebGlProgram, JsValue> {
    let program = context
        .create_program()
        .ok_or_else(|| 
            String::from("Unable to create shader object")
        )?;
    context.attach_shader(&program, vert_shader);
    context.attach_shader(&program, frag_shader);
    context.link_program(&program);
    if context
        .get_program_parameter(&program, WebGl2RenderingContext::LINK_STATUS)
        .as_bool()
        .unwrap_or(false)
    {
        Ok(program)
    } else {
        Err(context
            .get_program_info_log(&program)
            .unwrap_or_else(|| 
                String::from("Unknown error creating program object")
            )
            .into()
        )
    }
}

pub fn draw(context: &WebGl2RenderingContext, vert_count: i32) {
    context.clear_color(0.0, 0.0, 0.0, 1.0);
    context.clear(WebGl2RenderingContext::COLOR_BUFFER_BIT);

    context.draw_arrays(WebGl2RenderingContext::TRIANGLES, 0, vert_count);
}