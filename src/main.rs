use wgpu::{InstanceDescriptor};
use winit::event::{Event, MouseButton, VirtualKeyCode, WindowEvent};
use winit::event_loop;
use winit::event_loop::ControlFlow;
use winit::window;

mod game;

/*
* Following this tutorial: https://zdgeier.com/wgpuintro.html, but it seems to be a little bit old
* It's also unfinished :D
* Good Luck!
*
* Also following: https://sotrh.github.io/learn-wgpu/beginner/tutorial2-surface/#state-new
*/

struct DisplayState {
    surface: wgpu::Surface,
    device: wgpu::Device,
    queue: wgpu::Queue,
    config: wgpu::SurfaceConfiguration,
    size: winit::dpi::PhysicalSize<u32>,
    window: window::Window,

    // Not from the tutorial; experimentation
    screen_color: wgpu::Color,
    sample_app: game::GameBoard
}

impl DisplayState {
    // Creating some of the wgpu types requires async code
    async fn new(window: window::Window) -> Self {
        let size = window.inner_size();

        // Idk exactly what an instance is but
        let instance = wgpu::Instance::new(
            InstanceDescriptor{
                backends: wgpu::Backends::all(),
                dx12_shader_compiler: Default::default()
            }
        );

        let surface = unsafe { instance.create_surface(&window) }.unwrap();

        // This happens async, we're requesting a handle to the gpu
        let adapter = instance.request_adapter(
            &wgpu::RequestAdapterOptionsBase {
                power_preference: wgpu::PowerPreference::default(),
                compatible_surface: Some(&surface), //What does Some(T) do????????
                force_fallback_adapter: false,
            }
        ).await.unwrap();

        let (device, queue) = adapter.request_device(
            &wgpu::DeviceDescriptor {
                label: None,
                features: wgpu::Features::empty(),
                limits: wgpu::Limits::default(),
            },
            None,
        ).await.unwrap();

        let capabilities = surface.get_capabilities(&adapter);
        let config = wgpu::SurfaceConfiguration{
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: capabilities.formats[0], // Might have issue, tutorial checked if gpu supported sRGB surface
            width: size.width,
            height: size.height,
            present_mode: capabilities.present_modes[0], // should be PresentMode::Fifo
            alpha_mode: capabilities.alpha_modes[0],
            view_formats: vec![] // TextureFormat s that can be used to create TextureView s
        };
        surface.configure(&device, &config); // Configure the surface using the newly made config

        let color = wgpu::Color::BLACK;
        Self{
            surface,
            device,
            queue,
            config,
            size,
            window,
            screen_color: color,
            sample_app: game::GameBoard::default()
        }
    }

    pub fn window(&self) -> &window::Window {
        &self.window
    }

    fn resize(&mut self, new_size: winit::dpi::PhysicalSize<u32>) {
        if new_size.width > 0 && new_size.height > 0{
            self.size = new_size;
            self.config.width = new_size.width;
            self.config.height = new_size.height;
            self.surface.configure(&self.device, &self.config);
        }
    }

    /// Whether an event is still processing
    fn input(&mut self, event: &WindowEvent) -> bool {
        match event{
            WindowEvent::CursorMoved {position, ..} => {
                self.screen_color = wgpu::Color{
                    r: 0.5,
                    g: position.x as f64 / self.size.width as f64,
                    b: position.y as f64 / self.size.height as f64,
                    a: 1.0
                };
                true
            },

            WindowEvent::MouseInput {button, ..} => {
                match button{
                    MouseButton::Left => self.screen_color = wgpu::Color::BLACK,
                    MouseButton::Right => {
                        println!("Cell is {}", self.sample_app.get(0, 0))
                    },
                    _ => {}
                }
                true
            },

            _ => false
        }
    }

    fn update(&mut self) {
        //TODO: we apparently have nothing to update
    }

    fn render(&mut self) -> Result<(), wgpu::SurfaceError> { //IDK where the error variants would get returned
        let output = self.surface.get_current_texture()?; // Creating a texture for the render output
        let view = output.texture.create_view(&wgpu::TextureViewDescriptor::default()); //idk

        // Creating a buffer of commands that will eventually get sent to the gpu
        let mut encoder = self.device.create_command_encoder(
            &wgpu::CommandEncoderDescriptor {
                label: Some("Render Encoder"),
            }
        );

        let _render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: Some("Render Pass"),
            // I think: an array slice of Option<wgpu::RenderPassColorAttachment>
            // Where we will draw stuff (e.g. what texture, ...)
            color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                view: &view, // which texture to save the colors to
                resolve_target: None, // texture that receives the output, same as view (unless 'multisampling')

                // What to do w/ the texture specified
                ops: wgpu::Operations {
                    load: wgpu::LoadOp::Clear(self.screen_color), // replace the texture w/ this color on load
                    store: true, // store the result of the render to the texture
                },
            })],
            depth_stencil_attachment: None,
        });
        // begin_render_pass() has a &mut self, so calling the function borrows it
        // finish() below also has an &mut self, and you can only have one mutable reference,
        // so we have to drop (clear/delete/i think) render_pass
        drop(_render_pass);

        self.queue.submit(std::iter::once(encoder.finish())); //finish buffer & add it to queue
        output.present();
        return Ok(());
    }
}

async fn run(){
    env_logger::init(); // wGPU logging
    let event_loop = event_loop::EventLoop::new();
    let window = window::WindowBuilder::new().build(&event_loop).unwrap();

    // create the state struct
    let mut state = pollster::block_on(DisplayState::new(window));

    // Creating a closure (lambda) to handle all of the incoming events
    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Wait; // dereference b/c control_flow it &mut ControlFlow, not ControlFlow (I think)

        match event{ // Matching the event
            Event::WindowEvent {event: window_event, window_id} if window_id == state.window().id() => {
                if state.input(&window_event){ // Don't do anything else if the last event is processing
                    return ();
                }
                match window_event {
                    WindowEvent::CloseRequested => *control_flow = ControlFlow::Exit,

                    WindowEvent::KeyboardInput {input, ..} => {
                        match input.virtual_keycode {
                            Some(VirtualKeyCode::Escape) => *control_flow = ControlFlow::Exit,
                            _ => {}
                        }
                    },

                    WindowEvent::Resized(new_size) => state.resize(new_size), //Tutorial dereferences new_size

                    _ => {}
                }
            },

            Event::RedrawRequested(window_id) if window_id == state.window().id() => {
                state.update();
                match state.render() {
                    Ok(_) => {},
                    Err(wgpu::SurfaceError::Lost) => state.resize(state.size), // IDK
                    Err(wgpu::SurfaceError::OutOfMemory) => *control_flow = ControlFlow::Exit,
                    Err(e) => eprintln!("{:?}", e)
                };
            },

            Event::MainEventsCleared => state.window().request_redraw(),

            _ => {}
        }
    });
}

fn main() {
    println!("Hello, world!");
    pollster::block_on(run());
}
