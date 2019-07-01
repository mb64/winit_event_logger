fn main() {
    let event_loop = winit::event_loop::EventLoop::new();

    let window = winit::window::WindowBuilder::new().build(&event_loop);

    event_loop.run(|event, _, cf| {
        println!("The winit event is {:#?}", event);
        *cf = winit::event_loop::ControlFlow::Wait;
    });
}
