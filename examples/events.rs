use Gear::*;

fn main() {
    // create the engine with the window
    let mut engine = Engine::new() // creates the engine
        .with_gl_window(None); // with a window

    // create a renderer and give shaders to it
    let mut renderer = DefaultOpenGlRenderer::new();
    let program = ShaderProgram::simple_program(
        MONOCHROME_LIT_FRAG_SHADER,
        DEFAULT_VERT_SHADER
    ).expect("Unable to compile shaders !");

    // register the shader program in the renderer
    renderer.register_shader_program(program);

    // assign the renderer to the window
    match engine.get_gl_window_mut() {
        Some(window) => {
            window.set_new_renderer(Box::new(renderer));
        },
        None => {},
    }

    // create cube and camera entity
    let world = engine.get_world();

    // create an input event listener
    let mut input_listener = EventListener::new();
    input_listener.listen(EngineEvents::WindowEvent(glfw::WindowEvent::Close), Box::new(|event, _| match event { EngineEvents::WindowEvent(e) => println!("{:?}", e), _ => {}}));
    let _listening_entity = create_entity!(&mut world.components; input_listener);

    // start main loop
    engine.main_loop();

}




