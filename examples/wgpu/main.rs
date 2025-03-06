use winit::application::ApplicationHandler;
use winit::dpi::LogicalSize;
use winit::event::{MouseScrollDelta, WindowEvent};
use winit::event_loop::{ActiveEventLoop, ControlFlow, EventLoop};
use winit::window::{Window, WindowId};

use std::cell::RefCell;
use std::rc::Rc;

use clay_layout::Clay;

use ui_renderer::UIState;
mod graphics_context;
mod ui_layout;
mod ui_renderer;

#[rustfmt::skip]
fn main() {
    let event_loop = match EventLoop::new() {
        Ok(event_loop) => event_loop,
        Err(_) => return
    };
    event_loop.set_control_flow(ControlFlow::Wait);

    event_loop.run_app(&mut App::default()).unwrap();
}

use graphics_context::GraphicsContext;

#[derive(Default)]
pub struct App<'a> {
    ctx: Option<GraphicsContext<'a>>,

    pub ui_state: Option<Rc<RefCell<UIState>>>,
    pub clay: Option<Clay>,
    pub clay_user_data: ui_layout::ClayState,
}

impl<'a> ApplicationHandler for App<'a> {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        let window_attributes = Window::default_attributes()
            .with_title("Clay-rs-WGPU-Demo".to_string())
            .with_inner_size(LogicalSize::new(800, 600));

        let window = event_loop.create_window(window_attributes).unwrap();
        let size = window.inner_size();
        let dpi_scale = window.scale_factor() as f32;

        let ctx = GraphicsContext::new(window);

        let ui_state = Rc::<RefCell<UIState>>::new(RefCell::new(UIState::new(
            &ctx.device,
            &ctx.queue,
            ctx.config.format,
            size,
            dpi_scale,
        )));

        let mut clay = Clay::new((size.width as f32, size.height as f32).into());
        clay.set_debug_mode(false);

        clay.set_measure_text_function_user_data(ui_state.clone(), ui_layout::measure_text);

        ui_layout::initialize_user_data(&mut self.clay_user_data);

        self.ctx = Some(ctx);
        self.ui_state = Some(ui_state);
        self.clay = Some(clay);
    }

    fn window_event(&mut self, event_loop: &ActiveEventLoop, _id: WindowId, event: WindowEvent) {
        match event {
            WindowEvent::CloseRequested => {
                event_loop.exit();
            }
            WindowEvent::Resized(size) => {
                self.ctx.as_mut().unwrap().resize();
                self.ui_state
                    .as_mut()
                    .unwrap()
                    .borrow_mut()
                    .resize((size.width as i32, size.height as i32));
                self.clay_user_data.size = (size.width as f32, size.height as f32);
            }
            WindowEvent::ScaleFactorChanged {
                scale_factor,
                inner_size_writer: _,
            } => {
                self.ui_state.as_mut().unwrap().borrow_mut().dpi_scale = scale_factor as f32;
            }
            WindowEvent::RedrawRequested => {
                let render_commands = ui_layout::create_layout(
                    self.clay.as_mut().unwrap(),
                    &mut self.clay_user_data,
                    0.016,
                );
                let mut ui_renderer = self.ui_state.as_mut().unwrap().borrow_mut();

                self.ctx
                    .as_mut()
                    .unwrap()
                    .render(|mut render_pass, device, queue, config| {
                        ui_renderer.render_clay(
                            render_commands,
                            &mut render_pass,
                            &device,
                            &queue,
                            &config,
                        );
                    })
                    .unwrap();
                self.clay_user_data.mouse_down_rising_edge = false;
                self.ctx.as_ref().unwrap().window.request_redraw();
            }
            WindowEvent::MouseInput {
                device_id: _,
                state,
                button,
            } => match button {
                winit::event::MouseButton::Left => {
                    self.clay_user_data.mouse_down_rising_edge = state.is_pressed();
                }
                _ => {}
            },
            WindowEvent::MouseWheel {
                device_id: _,
                delta,
                phase: _,
            } => {
                self.clay_user_data.scroll_delta = match delta {
                    MouseScrollDelta::LineDelta(x, y) => (x, y),
                    MouseScrollDelta::PixelDelta(position) => position.into(),
                };
            }
            WindowEvent::CursorMoved {
                device_id: _,
                position,
            } => {
                self.clay_user_data.mouse_position = position.into();
            }
            _ => (),
        }
    }
}
