use winit::dpi::PhysicalSize;
use winit::event::{Event, WindowEvent};
use winit::event_loop::{ControlFlow, EventLoop};
use winit::window::WindowBuilder;

fn main() {
    let event_loop = EventLoop::new();
    let builder = WindowBuilder::new();
    let mut window = builder.build(&event_loop).unwrap();

    // window.set_cursor_visible(false);
    window.set_inner_size(PhysicalSize::new(400, 200));

    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Wait;

        if matches!(event,
        Event::WindowEvent{
                event:WindowEvent::CloseRequested,
                window_id
            }if window_id==window.id())
        {
            *control_flow = ControlFlow::Exit
        }

        if matches!(
            event,
            Event::WindowEvent {
                event: WindowEvent::MouseInput {
                    device_id,
                    state,
                    button,
                    modifiers,
                },
                window_id
            }
        ) {
            window.set_title("Mouse Pressed");
        }
    });
}
