use wgpu::util::DeviceExt;
use winit::{
    event::{Event, WindowEvent},
    platform::windows::WindowBuilder,
    window::Window,
    event_loop::{EventLoop, ControlFlow},
};

pub async fn benchmark_gpu() -> Duration {
    let event_loop = EventLoop::new();
    let window = WindowBuilder::new()
        .build(&event_loop)
        .expect("Failed to create window");

    let instance = wgpu::Instance::new(wgpu::Backends::all());
    let surface = unsafe { instance.create_surface(&window) };
    let adapter = instance
        .request_adapter(&wgpu::RequestAdapterOptions {
            power_preference: wgpu::PowerPreference::default(),
            compatible_surface: Some(&surface),
        })
        .await
        .expect("Failed to find an appropriate adapter");

    let (device, queue) = adapter
        .request_device(&wgpu::DeviceDescriptor {
            features: wgpu::Features::empty(),
            limits: wgpu::Limits::default(),
            shader_validation: true,
        }, None)
        .await
        .expect("Failed to create device");

    // Shader code: Simple color manipulation
    let shader_code = r#"
    #version 450
    layout(location = 0) out vec4 fragColor;
    void main() {
        fragColor = vec4(0.5 + 0.5 * sin(gl_FragCoord.x * 0.01), 
                         0.5 + 0.5 * cos(gl_FragCoord.y * 0.01), 
                         0.5 + 0.5 * sin(gl_FragCoord.x * 0.02 + gl_FragCoord.y * 0.02), 
                         1.0);
    }
    "#;

    let shader = device.create_shader_module(&wgpu::ShaderModuleDescriptor {
        label: Some("Color Shader"),
        source: wgpu::ShaderSource::Wgsl(shader_code.into()),
    });

    // Set up pipeline
    let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
        bind_group_layouts: &[],
        push_constant_ranges: &[],
    });

    let render_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
        label: Some("Render Pipeline"),
        layout: Some(&pipeline_layout),
        vertex: wgpu::VertexState {
            module: &shader,
            entry_point: "main",
            buffers: &[],
        },
        fragment: Some(wgpu::FragmentState {
            module: &shader,
            entry_point: "main",
            targets: &[wgpu::ColorTargetState {
                format: surface.get_preferred_format(&adapter),
                blend: None,
                write_mask: wgpu::ColorWrite::ALL,
            }],
        }),
        primitive: wgpu::PrimitiveState {
            topology: wgpu::PrimitiveTopology::TriangleList,
            ..Default::default()
        },
        depth_stencil: None,
        multisample: wgpu::MultisampleState {
            count: 1,
            ..Default::default()
        },
        multiview: None,
    });

    // Frame rendering
    let start = Instant::now();

    let mut swap_chain = device.create_swap_chain(&surface, &wgpu::SwapChainDescriptor {
        usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
        format: surface.get_preferred_format(&adapter),
        width: window.inner_size().width,
        height: window.inner_size().height,
        present_mode: wgpu::PresentMode::Fifo,
    });

    event_loop.run(move |event, _, control_flow| {
        match event {
            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                ..
            } => {
                *control_flow = ControlFlow::Exit;
            }
            Event::RedrawRequested(_) => {
                let frame = swap_chain.get_current_frame().unwrap().output;
                let mut encoder = device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
                    label: Some("Render Encoder"),
                });

                let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                    label: Some("Render Pass"),
                    color_attachments: &[wgpu::RenderPassColorAttachment {
                        view: &frame.texture.create_view(&wgpu::TextureViewDescriptor::default()),
                        resolve_target: None,
                        ops: wgpu::Operations {
                            load: wgpu::LoadOp::Clear(wgpu::Color::BLACK),
                            store: true,
                        },
                    }],
                    depth_stencil_attachment: None,
                });

                render_pass.set_pipeline(&render_pipeline);
                render_pass.draw(0..3, 0..1);

                queue.submit(Some(encoder.finish()));

                window.request_redraw();
            }
            _ => {}
        }
    });

    start.elapsed()
}
