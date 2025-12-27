use glium::{Surface};
use std::{env, fs::{self, metadata}, io::{self, ErrorKind}, path::{Path, PathBuf}, time::{SystemTime, Instant}};



#[derive(Debug, Clone, Copy)]
struct Vertex
{
    position: [f32; 2],
}
implement_vertex!(Vertex, position);



fn parse_args() -> Result<(String, String), String> {
    let args: Vec<String> = env::args().collect();
    
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

    let mut vert_path = String::new();
    let mut frag_path = String::new();

    match parse_args()
    {
        Ok((f, v)) => (frag_path, vert_path) = (expand_tilde(f.trim().to_string()), expand_tilde(v.trim().to_string())),
        Err(e) => println!("{e}")
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


    let event_loop = glium::winit::event_loop::EventLoopBuilder::new().build().expect("Event loop building");
    let ( window, display ) = glium::backend::glutin::SimpleWindowBuilder::new().build(&event_loop);

    window.set_title("ShaderShader");

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

    let mut meta_time: SystemTime = SystemTime::now(); 

    let mut program = glium::Program::from_source(&display, vert_shader, frag_shader, None).unwrap();

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
                if check_shader_refresh(&vert_path, &frag_path, &mut meta_time)
                {
                    meta_time= SystemTime::now(); 

                    if let Ok(p) = glium::Program::from_source(&display, &vert_source.as_str(), &frag_source.as_str(), None)
                    {
                        program = p;
                    }
                    else 
                    {
                        println!("ERROR! Shader failed to compile!");   
                    }
                }

                window.request_redraw();    
            },

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
        println!("Please provide path of vertex shader (default: ~/shadershader/vertex_shader.glsl)");
    
        io::stdin().read_line(&mut vert_in).unwrap();
        vert_in = expand_tilde(vert_in.trim().to_string());

        if vert_in.is_empty()
        {
            vert_in = expand_tilde("~/shadershader/vertex_shader.glsl".to_string());
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
        println!("Please provide path of frag shader (default: ~/shadershader/fragment_shader.glsl)");
    
        io::stdin().read_line(&mut frag_in).unwrap();
        frag_in = expand_tilde(frag_in.trim().to_string());

        if frag_in.is_empty()
        {
            frag_in = expand_tilde("~/shadershader/fragment_shader.glsl".to_string());
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



fn check_shader_refresh(vert: &String, frag: &String, time: &mut SystemTime) -> bool
{

    let vert_met = metadata(vert).unwrap();
    let frag_met = metadata(frag).unwrap();
    let mut modified = vert_met.modified().unwrap();

    if modified > *time
    {
        *time = modified;
        return true;
    }

    modified = frag_met.modified().unwrap();

    if modified > *time
    {
        *time = modified;
        return true;
    }



    false

}
