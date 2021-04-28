use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{WebGlProgram, WebGlRenderingContext, WebGlShader};
use crate::canvas;

#[wasm_bindgen]
pub fn draw_triangle() -> Result<(), JsValue> {
    let context: WebGlRenderingContext = canvas::create_context()?; // create our webgl context
    //store our vertices that will be used to make the triangle
    let vertices: Vec<f32> = vec![-0.5, -0.5, 0.0, 0.5, -0.5, 0.0, 0.0, 0.5, 0.0];


    // Create a buffer to store the vertices
    let buffer = context.create_buffer().ok_or("Failed to Create Buffer")?;

    context.bind_buffer(WebGlRenderingContext::ARRAY_BUFFER, Some(&buffer));

    unsafe {
        // this vert_array is unsafe because it's a view directly into WASM memory
        // updating memory may invalidate the view
        let vert_array = js_sys::Float32Array::view(&vertices);

        //load the data into the buffer
        context.buffer_data_with_array_buffer_view(
            WebGlRenderingContext::ARRAY_BUFFER,
            &vert_array,
            WebGlRenderingContext::STATIC_DRAW,
        );
    }

    // Make our vertex shader
    let vertex_shader = r#"
    attribute vec4 position;

    void main() {
        gl_Position = position;
    }"#;
    let vertex_shader = compile_shader(&context, WebGlRenderingContext::VERTEX_SHADER, vertex_shader)?;

    let fragment_shader = r#"
    void main() {
        gl_FragColor = vec4(1.0, 1.0, 1.0, 1.0);
    }
    "#;
    let fragment_shader = compile_shader(&context, WebGlRenderingContext::FRAGMENT_SHADER, fragment_shader)?;

    // link the shaders to a program
    let program = link_shaders(&context, vertex_shader, fragment_shader)?;
    context.use_program(Some(&program)); // need to tell WebGL to use the program

    // now we need to tell GL how it's supposed to draw our points
    context.vertex_attrib_pointer_with_i32(0, 3, WebGlRenderingContext::FLOAT, false, 0, 0);
    context.enable_vertex_attrib_array(0);

    // pick a color that clears the screen and clear it
    context.clear_color(0.0, 0.0, 0.0, 1.0);
    context.clear(WebGlRenderingContext::COLOR_BUFFER_BIT);

    // tell opengl that the coordinates are for triangles
    context.draw_arrays(
        WebGlRenderingContext::TRIANGLES,
        0,
        (vertices.len() / 3) as i32,
    );
    Ok(()) // return ok if there's no error
}

fn compile_shader(context: &WebGlRenderingContext, shader_type: u32, source: &str) -> Result<WebGlShader, String> {
    // need to create a shader, load the source into the shader, and then compile
    let shader = context.create_shader(shader_type).ok_or_else(|| String::from("Failed to create Shader"))?;
    context.shader_source(&shader, source);
    context.compile_shader(&shader);

    if context
        .get_shader_parameter(&shader, WebGlRenderingContext::COMPILE_STATUS)
        .as_bool()
        .unwrap_or(false)
    {
        Ok(shader)
    } else {
        Err(context
            .get_shader_info_log(&shader)
            .unwrap_or_else(|| String::from("Unknown error creating shader")))
    }
}

fn link_shaders(context: &WebGlRenderingContext, vertex_shader: WebGlShader, fragment_shader: WebGlShader) -> Result<WebGlProgram, String>{
    let program = context.create_program().ok_or_else(|| String::from("Could not Create shader program"))?;
    context.attach_shader(&program, &vertex_shader);
    context.attach_shader(&program, &fragment_shader);
    context.link_program(&program);

    // Check that the context was linked
    if context.get_program_parameter(&program, WebGlRenderingContext::LINK_STATUS)
        .as_bool().unwrap_or(false){
        Ok(program)
    } else{
        Err(String::from("Could not Link Shader, unknown error"))
    }
}