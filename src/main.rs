use wgpu::{CompositeAlphaMode, InstanceDescriptor, TextureFormat};
use winit::event::{Event, VirtualKeyCode, WindowEvent};
use winit::event_loop;
use winit::event_loop::ControlFlow;
use winit::window;

/*
* Following this tutorial: https://zdgeier.com/wgpuintro.html, but it seems to be a little bit old
* Good Luck!
*/

fn main() {
    println!("Hello, world!");

    env_logger::init(); // wGPU logging
    let event_loop = event_loop::EventLoop::new();
    let window = window::WindowBuilder::new().build(&event_loop).unwrap();

    let instance = wgpu::Instance::new(InstanceDescriptor::default()); // Not the tutorial
    let surface = unsafe { instance.create_surface(&window) }.unwrap();

    let adapter = pollster::block_on(
        // Waiting until this code is ready & run (async stuff b/c hardware)
        instance.request_adapter(&wgpu::RequestAdapterOptionsBase {
            power_preference: wgpu::PowerPreference::default(),
            compatible_surface: Some(&surface), //What does Some(T) do????????
            force_fallback_adapter: false,
        }),
    )
    .unwrap();

    let (device, queue) = pollster::block_on(adapter.request_device(
        &wgpu::DeviceDescriptor {
            label: None,
            features: wgpu::Features::empty(),
            limits: wgpu::Limits::default(),
        },
        None,
    ))
    .unwrap();

    let size = window.inner_size();
    surface.configure(
        &device,
        &wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: TextureFormat::Bgra8Unorm, // No idea what this does, picked one at random
            view_formats: vec![TextureFormat::Bgra8Unorm], // the tutorial doesn't have these structs in it
            width: size.width,
            height: size.height,
            present_mode: wgpu::PresentMode::Fifo,
            alpha_mode: CompositeAlphaMode::Auto,
        },
    );

    // Cycle colors
    let mut blue_inc = 0;
    let mut blue_val = 0f64;
    // Creating a lambda to handle all of the incoming events from the window
    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Wait; // Why need to dereference here ?

        match event {
            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                window_id,
            } if window_id == window.id() => *control_flow = ControlFlow::Exit,

            Event::WindowEvent {
                event: WindowEvent::KeyboardInput { input, .. },
                window_id,
            } if window_id == window.id() => {
                if input.virtual_keycode == Some(VirtualKeyCode::Escape) {
                    *control_flow = ControlFlow::Exit;
                }
            }

            Event::RedrawRequested(_) => {
                // Redrawing when any window gets a requested redraw I think??
                let output = surface.get_current_texture().unwrap();
                let view = output
                    .texture
                    .create_view(&wgpu::TextureViewDescriptor::default());
                let mut encoder = device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
                    label: Some("Render Encoder"),
                });

                {
                    //Inlaid scope makes it so encoder isn't moved (idk where the move happened either way but)
                    let _render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                        label: Some("Render Pass"),
                        color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                            //yeah absolutely no clue
                            view: &view,
                            resolve_target: None,
                            ops: wgpu::Operations {
                                load: wgpu::LoadOp::Clear(wgpu::Color {
                                    r: 0.5,
                                    g: 0.05,
                                    b: blue_val,
                                    a: 1.0,
                                }),
                                store: true,
                            },
                        })],
                        depth_stencil_attachment: None,
                    });
                }
                queue.submit(std::iter::once(encoder.finish()));
                output.present();

                blue_val = (blue_inc % 64) as f64 / 64.0;
                blue_inc += 1;
            }

            Event::MainEventsCleared => window.request_redraw(),

            _ => {}
        }
    });
}
