use glium::{Surface, glutin::config::ConfigTemplateBuilder};
use notify::{Event, Watcher};
use std::{env, fs::{self}, io::{self, ErrorKind}, path::{Path, PathBuf}, sync::mpsc, time::Instant}; 



#[derive(Debug, Clone, Copy)]
struct Vertex
{
    position: [f32; 2],
}
implement_vertex!(Vertex, position);



fn parse_args() -> Result<(String, String), String> {
    let args: Vec<String> = env::args().collect();
   
    if args.contains(&"-h".to_string())
    {
        println!(r#"
        Usage:

        shadershader [options]

        Options:

        -h         How to use shadershader
        -f <path>  Fragment shader path
        -v <path>  Vertex Shader path
        "#);

        return Err(String::new())
    }

    let mut frag_path = String::new();
    let mut vert_path = String::new();
    
    let mut i = 1;
    while i < args.len() {
        match args[i].as_str() {
            "-f" => {
                if i + 1 >= args.len() {
                    return Err("Error: -f requires a path argument".to_string());
                }
                frag_path = args[i + 1].clone();
                i += 2;
            }
            "-v" => {
                if i + 1 >= args.len() {
                    return Err("Error: -v requires a path argument".to_string());
                }
                vert_path = args[i + 1].clone();
                i += 2; 
            }
            _ => {
                return Err(format!("Error: Unknown argument '{}'", args[i]));
            }
        }
    }
    
    Ok((frag_path, vert_path))
}



#[macro_use]
extern crate glium;
fn main() {

    // Obtain file paths
    let mut vert_path: String;
    let mut frag_path: String;

    match parse_args()
    {
        Ok((f, v)) => (frag_path, vert_path) = (expand_tilde(f.trim().to_string()), expand_tilde(v.trim().to_string())),
        Err(e) => {println!("{e}"); return}
    }


    if vert_path.is_empty() && frag_path.is_empty()
    {
        (vert_path, frag_path) = obtain_files();
    }
    else if vert_path.is_empty() 
    {
        vert_path = obtain_vertex_file();    
    }
    else if frag_path.is_empty() 
    {
        frag_path = obtain_fragment_file();    
    }


    if !fs::exists(expand_tilde("~/.config/shadershader/".to_string())).unwrap()
    {
        fs::create_dir_all(expand_tilde("~/.config/shadershader/".to_string())).unwrap()
    }

    let vert_parent_path = Path::new(&vert_path).parent().unwrap();
    let frag_parent_path = Path::new(&frag_path).parent().unwrap();

    // Set up watchers
    let (tx_frag, rx_frag) = mpsc::channel::<notify::Result<Event>>();
    let (tx_vert, rx_vert) = mpsc::channel::<notify::Result<Event>>();

    let mut frag_watcher = match notify::recommended_watcher(tx_frag)
    {
        Ok(watcher) => watcher,
        Err(e) => {println!("ERROR Failed to construct fragment shader watcher: {e}"); return}
    };
    let mut vert_watcher = match notify::recommended_watcher(tx_vert)
    {
        Ok(watcher) => watcher,
        Err(e) => {println!("ERROR Failed to construct vertex shader watcher: {e}"); return}
    };

    match frag_watcher.watch(Path::new(&frag_parent_path), notify::RecursiveMode::NonRecursive)
    {
        Ok(_) => {},
        Err(e) => {println!("ERROR Failed to watch fragment shader path: {e}"); return}
    };
    match vert_watcher.watch(Path::new(&vert_parent_path), notify::RecursiveMode::NonRecursive)
    {
        Ok(_) => {},
        Err(e) => {println!("ERROR failed to watch vertex shader: {e}"); return}
    }


    // The meat and potatoes
    let event_loop = glium::winit::event_loop::EventLoopBuilder::new().build().expect("Event loop building");
    let ( window, display ) = glium::backend::glutin::SimpleWindowBuilder::new().with_config_template_builder(
        ConfigTemplateBuilder::new()
            .prefer_hardware_accelerated(Some(true))
            .with_multisampling(2))
            .with_title("ShaderShader")
        .build(&event_loop
    );

    

    let vert1 = Vertex { position: [ -1.0, -1.0 ]};
    let vert2 = Vertex { position: [ -1.0,  1.0 ]};
    let vert3 = Vertex { position: [  1.0, -1.0 ]};
    let vert4 = Vertex { position: [  1.0,  1.0 ]};
    let shape = vec![vert1, vert2, vert3, vert4];

    let vert_buffer = glium::VertexBuffer::new(&display, &shape).unwrap();
    
    let indices = glium::index::IndexBuffer::new(&display, glium::index::PrimitiveType::TriangleStrip, &[0u16, 1, 2, 3]).unwrap();



    let vert_source = load_shader(vert_path.as_str()).unwrap();
    let frag_source = load_shader(frag_path.as_str()).unwrap();

    let vert_shader = vert_source.as_str();
    let frag_shader = frag_source.as_str();

    let mut program = match glium::Program::from_source(&display, vert_shader, frag_shader, None) {
        Ok(p) => p,
        Err(e) => {println!("ERROR Failed to compile OpenGL program. Please fix any error before running ShaderShader: {e}"); return},
    };
    
    // Git 'er goin'
    let start_time = Instant::now();
    let _ = event_loop.run(move |event, window_target|
    {

        match event {
            
            glium::winit::event::Event::WindowEvent { event, .. } => match event
            {
                glium::winit::event::WindowEvent::CloseRequested => window_target.exit(),
                glium::winit::event::WindowEvent::RedrawRequested =>
                {

                    let time = start_time.elapsed().as_secs_f32();

                    let uniforms = uniform!
                    {
                        iTime: time,
                        iResolution: [window.inner_size().width as f32, window.inner_size().height as f32, 1.0],
                    };
                
                    let mut target = display.draw();
                    target.clear_color(0.0, 0.0, 0.0, 1.0);
                    target.draw(&vert_buffer, &indices, &program, &uniforms, &Default::default()).unwrap();
                    target.finish().unwrap();
                
                },
                glium::winit::event::WindowEvent::Resized(window_size) =>
                {
                    display.resize(window_size.into());
                }
                _ => (),
            
            },

            glium::winit::event::Event::AboutToWait =>
            {

                let mut do_refresh = false;

                if let Ok(Ok(event)) = rx_frag.try_recv()
                {
                    do_refresh |= event.kind.is_modify() 
                }
                if let Ok(Ok(event)) = rx_vert.try_recv()
                {
                    do_refresh |= event.kind.is_modify()
                }


                if do_refresh 
                {
                    let new_frag = load_shader(&frag_path.as_str()).unwrap_or_default();
                    let new_vert = load_shader(&vert_path.as_str()).unwrap_or_default();

                    if let Ok(p) = glium::Program::from_source(&display, &new_vert, &new_frag, None)
                    {
                        program = p;
                    }
                    else 
                    {
                        println!("ERROR Shader failed to compile!");   
                    }
                } // if do_redraw
                
                window.request_redraw();    


            }, // glium::winit::winit::Event::AboutToWait

            _ => (),
        
        };

    });

}



fn load_shader(path: &str) -> Result<String, ErrorKind>
{
    match fs::read_to_string(path) {
        Ok(str) => return Ok(str),
        Err(e) =>
        {
            println!("Failed to read file from path: {path}\nError: {e}");
            return Err(e.kind());
        }
    };
}



fn obtain_vertex_file() -> String
{
    let mut vert_in = String::new();

    loop
    {    
        vert_in.clear();
        println!("Please provide path of vertex shader (default: ~/.config/shadershader/vertex_shader.glsl)");
    
        io::stdin().read_line(&mut vert_in).unwrap();
        vert_in = expand_tilde(vert_in.trim().to_string());

        if vert_in.is_empty()
        {
            vert_in = expand_tilde("~/.config/shadershader/vertex_shader.glsl".to_string());
        }

        if Path::new(&vert_in).is_file()
        {
            break;
        }
        else
        {
            println!("Path to vertex shader does not exist.");
        }
    }

    vert_in

}



fn obtain_fragment_file() -> String
{
    let mut frag_in = String::new();

    loop
    {    
        frag_in.clear();
        println!("Please provide path of frag shader (default: ~/.config/shadershader/fragment_shader.glsl)");
    
        io::stdin().read_line(&mut frag_in).unwrap();
        frag_in = expand_tilde(frag_in.trim().to_string());

        if frag_in.is_empty()
        {
            frag_in = expand_tilde("~/.config/shadershader/fragment_shader.glsl".to_string());
        }

        if Path::new(&frag_in).is_file()
        {
            break;
        }
        else
        {
            println!("Path to fragment shader does not exist.");
        }
    }

    frag_in
}



fn obtain_files() -> (String, String)
{
    (obtain_vertex_file(), obtain_fragment_file())
}



fn expand_tilde(path: String) -> String
{

    if path.starts_with("~")
    {
        if let Some(home) = env::var_os("HOME")
        {
            let mut expanded = PathBuf::from(home);
            expanded.push(&path[1..].trim_start_matches('/'));
            return expanded.to_str().unwrap().to_string();
        }
    }

    path

}
