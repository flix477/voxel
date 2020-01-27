mod entities;
mod input;
mod math;
mod models;
mod rendering;

use entities::{camera::Camera, Entity, Updatable};
use glium::glutin::{
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
};
use input::Input;
use models::primitives::{CUBE_INDICES, CUBE_VERTICES};
use rendering::{display::DisplayManager, loader::Loader, renderer::Renderer};
use std::time::{Duration, Instant};

fn main() {
    run();
}

fn run() {
    let mut event_loop = EventLoop::new();
    let mut dm = DisplayManager::new(&event_loop).unwrap();
    let mut renderer = Renderer::new(dm.display());
    let mut input = Input::default();
    let mut camera = Camera::default();

    let model = Loader::create_model(&CUBE_VERTICES, &CUBE_INDICES, dm.display());
    let mut entity = Entity::new(model);
    entity.position.set_z(2.0);
    let frame_delay = Duration::from_nanos(16_666_667);

    event_loop.run(move |event, _, control_flow| {
        let next_frame = Instant::now() + frame_delay;
        *control_flow = ControlFlow::WaitUntil(next_frame);

        if let Event::WindowEvent {
            event: WindowEvent::CloseRequested,
            ..
        } = &event
        {
            *control_flow = ControlFlow::Exit;
            return;
        }

        input.on_event(&event);

        camera.update(&input);

        renderer.render(dm.display_mut(), &entity, &camera);
    });
}
