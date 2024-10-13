use chrono::Local;
use egui::ViewportId;
use glium::Surface;
use winit::event;

fn main() {
    let width = 800;
    let height = 600;

    let event_loop = winit::event_loop::EventLoopBuilder::new().build().unwrap();
    let (window, display) = glium::backend::glutin::SimpleWindowBuilder::new()
        .with_title("Duck")
        .with_inner_size(width, height)
        .build(&event_loop);

    let mut egui_glium =
        egui_glium::EguiGlium::new(ViewportId::ROOT, &display, &window, &event_loop);

    let mut previous_time = Local::now();

    let _ = event_loop.run(move |event, window_target| {
        let mut redraw = || {
            let current_time = Local::now();
            let duration = current_time - previous_time;
            let duration_in_seconds = duration.num_microseconds().unwrap_or(1) as f64 / 1_000_000.0;
            let fps = 1.0 / duration_in_seconds;
            previous_time = current_time;

            egui_glium.run(&window, |egui_ctx| {
                egui::Window::new("panel").show(egui_ctx, |ui| {
                    ui.label(format!("FPS: {:.1}", fps));
                });
            });

            window.request_redraw();

            let mut target = display.draw();

            target.clear_color_and_depth((0.0, 0.0, 0.0, 1.0), 1.0);

            egui_glium.paint(&display, &mut target);

            target.finish().unwrap();
        };

        match event {
            event::Event::WindowEvent { event, .. } => {
                use event::WindowEvent;
                match &event {
                    WindowEvent::RedrawRequested => redraw(),
                    WindowEvent::CloseRequested | WindowEvent::Destroyed => {
                        window_target.exit();
                    }
                    WindowEvent::Resized(new_size) => {
                        display.resize((*new_size).into());
                        // perspective = Matrix4::new_perspective(
                        //     new_size.width as f32 / new_size.height as f32,
                        //     std::f32::consts::PI / 2.0,
                        //     0.1,
                        //     100.0,
                        // );
                    }
                    _ => {}
                }

                let event_response = egui_glium.on_event(&window, &event);

                if event_response.repaint {
                    window.request_redraw();
                }
            }
            event::Event::NewEvents(event::StartCause::ResumeTimeReached { .. }) => {
                window.request_redraw();
            }
            _ => (),
        }
    });
}
