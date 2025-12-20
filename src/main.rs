use glium::{Surface};
use std::{env, fs::{self, metadata}, io::{self, ErrorKind, Read}, path::{self, Path, PathBuf}, str::FromStr, time::{Duration, SystemTime}};



#[derive(Debug, Clone, Copy)]
struct Vertex
{
    position: [f32; 2],
    color: [f32; 3]
}
implement_vertex!(Vertex, position, color);


#[macro_use]
extern crate glium;
fn main() {
    println!("Hello, world!");

    let (vert_path, frag_path) = obtain_files();

    let event_loop = glium::winit::event_loop::EventLoopBuilder::new().build().expect("Event loop building");
    let ( window, display ) = glium::backend::glutin::SimpleWindowBuilder::new().build(&event_loop);
  

    // Triangle
    let vert1 = Vertex { position: [ 0.0,  0.5 ], color: [1.0, 0.0, 0.0] };
    let vert2 = Vertex { position: [-0.5, -0.5 ], color: [0.0, 1.0, 0.0] };
    let vert3 = Vertex { position: [ 0.5, -0.25], color: [0.0, 0.0, 1.0] };
    let shape = vec![vert1, vert2, vert3];

    let vert_buffer = glium::VertexBuffer::new(&display, &shape).unwrap();
    
    let indices = glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList);



    let mut vert_source = load_shader(vert_path.as_str()).unwrap();
    let mut frag_source = load_shader(frag_path.as_str()).unwrap();

    let mut vert_shader = vert_source.as_str();
    let mut frag_shader = frag_source.as_str();

    let mut meta_time: SystemTime = SystemTime::now(); 

    let mut program = glium::Program::from_source(&display, vert_shader, frag_shader, None).unwrap();

    let mut time: f32 = 0.0;
    let _ = event_loop.run(move |event, window_target|
    {

        match event {
            
            glium::winit::event::Event::WindowEvent { event, .. } => match event
            {
                glium::winit::event::WindowEvent::CloseRequested => window_target.exit(),
                glium::winit::event::WindowEvent::RedrawRequested =>
                {

                    time += 0.02;

                    let uniforms = uniform!
                    {
                        transform: [
                            [time.cos(), time.sin(), 0.0, 0.0],
                            [-time.sin(), time.cos(), 0.0, 0.0],
                            [0.0, 0.0, 1.0, 0.0],
                            [0.0 ,0.0, 0.0, 1.0f32]
                        ]

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
                    
                    vert_source = load_shader(vert_path.as_str()).unwrap();
                    frag_source = load_shader(frag_path.as_str()).unwrap();

                    meta_time= SystemTime::now(); 

                    program = glium::Program::from_source(&display, &vert_source.as_str(), &frag_source.as_str(), None).unwrap();
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

fn obtain_files() -> (String, String)
{
    let mut vert_in = String::new();
    let mut frag_in = String::new();

    let mut bad_read = true;

    loop
    {    
        vert_in.clear();
        println!("Please provide path of vertex shader (default: ~/vertex_shader.glsl)");
    
        io::stdin().read_line(&mut vert_in).unwrap();
        vert_in = expand_tilde(vert_in.trim().to_string());

        if Path::new(&vert_in).is_file()
        {
            break;
        }
        else
        {
            println!("Path to vertex shader does not exist.");
        }
    }

    
    loop
    {    
        frag_in.clear();
        println!("Please provide path of frag shader (default: ~/fragment_shader.glsl)");
    
        io::stdin().read_line(&mut frag_in).unwrap();
        frag_in = expand_tilde(frag_in.trim().to_string());

        if Path::new(&frag_in).is_file()
        {
            break;
        }
        else
        {
            println!("Path to fragment shader does not exist.");
        }
    }
    
    if vert_in == "".to_string()
    {
        vert_in = expand_tilde("~/vertex_shader.glsl".to_string());
    }
    if frag_in == "".to_string()
    {
        frag_in = expand_tilde("~/fragment_shader.glsl".to_string());
    }

    (vert_in, frag_in)

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
