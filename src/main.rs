pub mod vertex;
pub mod generate_block;
pub mod block_drawer;
pub mod height_map;

use block_drawer::BlockDrawer;
use chrono::Local;
use egui::{DragValue, ViewportId, Widget};
use generate_block::generate_block;
use glium::Surface;
use height_map::HeightMap;
use nalgebra::{Matrix4, Point3, Vector3, Vector4};
use winit::event::{self, ElementState, MouseButton};

fn main() {
    let width = 1600;
    let height = 1200;

    let event_loop = winit::event_loop::EventLoopBuilder::new().build().unwrap();
    let (window, display) = glium::backend::glutin::SimpleWindowBuilder::new()
        .with_title("Milling simulator")
        .with_inner_size(width, height)
        .build(&event_loop);

    let mut egui_glium =
        egui_glium::EguiGlium::new(ViewportId::ROOT, &display, &window, &event_loop);

    let drawing_parameters = glium::DrawParameters {
        depth: glium::Depth {
            test: glium::draw_parameters::DepthTest::IfLess,
            write: true,
            ..Default::default()
        },
        backface_culling: glium::draw_parameters::BackfaceCullingMode::CullClockwise,
        ..Default::default()
    };

    let mut perspective = Matrix4::new_perspective(
        width as f32 / height as f32,
        std::f32::consts::PI / 2.0,
        0.1,
        100.0,
    );

    let mut mouse_position = (0.0, 0.0);
    let mut camera_direction = Vector3::new(0.0f32, 0.0, 1.0);
    let mut camera_angle = Vector3::new(0.0f32, 0.0, 0.0);
    let mut camera_up = Vector3::new(0.0f32, 1.0, 0.0);
    let mut camera_distant = 20.0f32;
    let mut view = Matrix4::look_at_rh(
        &Point3::from_slice((-camera_distant * camera_direction).as_slice()),
        &Point3::new(0.0, 0.0, 0.0),
        &camera_up,
    );
    let mut camera_move_button_pressed = false;

    let mut block_size = (15.0, 5.0, 15.0);
    let mut block_resolution = (600, 600);
    let mut block = generate_block(block_size, block_resolution);
    let mut vertex_buffer = glium::VertexBuffer::new(&display, &block).unwrap();
    let block_drawer = BlockDrawer::new(&display);

    let mut height_map = HeightMap::new(block_resolution, block_size.1, &display);

    let mut block_created = false;

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
                    if !block_created {
                        ui.horizontal(|ui| {
                            ui.label("size x: ");
                            DragValue::new(&mut block_size.0).clamp_range(1.0..=20.0).speed(0.1).ui(ui);
                            ui.label("cm");
                        });
                        ui.horizontal(|ui| {
                            ui.label("size y: ");
                            DragValue::new(&mut block_size.1).clamp_range(1.0..=20.0).speed(0.1).ui(ui);
                            ui.label("cm");
                        });
                        ui.horizontal(|ui| {
                            ui.label("size z: ");
                            DragValue::new(&mut block_size.2).clamp_range(1.0..=20.0).speed(0.1).ui(ui);
                            ui.label("cm");
                        });

                        ui.horizontal(|ui| {
                            ui.label("resoultion x: ");
                            DragValue::new(&mut block_resolution.0).clamp_range(10..=1500).ui(ui);
                        });
                        ui.horizontal(|ui| {
                            ui.label("resoultion z: ");
                            DragValue::new(&mut block_resolution.1).clamp_range(10..=1500).ui(ui);
                        });

                        if ui.button("Create block").clicked() {
                            block = generate_block(block_size, block_resolution);
                            vertex_buffer = glium::VertexBuffer::new(&display, &block).unwrap();
                            height_map = HeightMap::new(block_resolution, block_size.1, &display);
                            block_created = true;
                        }
                    } else {
                        if ui.button("Reset").clicked() {
                            block_created = false;
                            height_map = HeightMap::new(block_resolution, block_size.1, &display);
                        }
                    }

                    ui.label(format!("FPS: {:.1}", fps));
                });
            });

            window.request_redraw();

            let mut target = display.draw();

            target.clear_color_and_depth((0.0, 0.0, 0.0, 1.0), 1.0);

            block_drawer.draw(&mut target, &vertex_buffer, &perspective, &view, &drawing_parameters, -camera_distant * camera_direction, height_map.get_texture());

            egui_glium.paint(&display, &mut target);

            target.finish().unwrap();

            for i in 0..100 {
                for j in 0..100 {
                    height_map.write((i, j), 2.0);
                }
            }

            height_map.update_texture();
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
                        perspective = Matrix4::new_perspective(
                            new_size.width as f32 / new_size.height as f32,
                            std::f32::consts::PI / 2.0,
                            0.1,
                            100.0,
                        );
                    }
                    WindowEvent::CursorMoved { position, .. } => {
                        let delta = (position.x - mouse_position.0, position.y - mouse_position.1);
                        mouse_position = (position.x, position.y);
                        if camera_move_button_pressed {
                            camera_angle.x += delta.1 as f32 * 0.01;
                            camera_angle.y += delta.0 as f32
                                * 0.01
                                * if camera_angle.x.cos() < 0.0 {
                                    -1.0
                                } else {
                                    1.0
                                };
                            camera_direction =
                                (Matrix4::from_euler_angles(camera_angle.x, camera_angle.y, 0.0)
                                    * Vector4::new(0.0, 0.0, 1.0, 0.0))
                                .xyz();
                            camera_up =
                                (Matrix4::from_euler_angles(camera_angle.x, camera_angle.y, 0.0)
                                    * Vector4::new(0.0, 1.0, 0.0, 0.0))
                                .xyz();
                            view = Matrix4::look_at_rh(
                                &Point3::from_slice(
                                    (-camera_distant * camera_direction).as_slice(),
                                ),
                                &Point3::new(0.0, 0.0, 0.0),
                                &camera_up,
                            );
                        }
                    }
                    WindowEvent::MouseInput { state, button, .. } => {
                        if *button == MouseButton::Middle {
                            camera_move_button_pressed = *state == ElementState::Pressed;
                        }
                    }
                    WindowEvent::KeyboardInput { device_id: _, event, is_synthetic: _ } => {
                        if event.logical_key == "c" && event.state.is_pressed() && !event.repeat {
                            camera_move_button_pressed = !camera_move_button_pressed;
                        }
                    }
                    WindowEvent::MouseWheel { delta, .. } => match delta {
                        event::MouseScrollDelta::LineDelta(_x, y) => {
                            camera_distant += -y * 0.1;
                            view = Matrix4::look_at_rh(
                                &Point3::from_slice(
                                    (-camera_distant * camera_direction).as_slice(),
                                ),
                                &Point3::new(0.0, 0.0, 0.0),
                                &camera_up,
                            );
                        }
                        _ => {}
                    },
                    WindowEvent::TouchpadMagnify { delta, .. } => {
                        camera_distant -= *delta as f32 * 3.0;
                        view = Matrix4::look_at_rh(
                            &Point3::from_slice(
                                (-camera_distant * camera_direction).as_slice(),
                            ),
                            &Point3::new(0.0, 0.0, 0.0),
                            &camera_up,
                        );
                    },
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
