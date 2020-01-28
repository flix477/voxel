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
use models::primitives::{CUBE_INDICES, CUBE_NORMALS, CUBE_VERTICES};
use rendering::{display::DisplayManager, loader::Loader, renderer::Renderer};
use std::rc::Rc;

fn main() {
    run();
}

fn run() {
    let event_loop = EventLoop::new();
    let mut dm = DisplayManager::new(&event_loop).unwrap();
    let mut renderer = Renderer::new(dm.display());
    let mut input = Input::default();
    let mut camera = Camera::default();

    let cube_model = Rc::new(Loader::create_model(
        &CUBE_VERTICES,
        &CUBE_INDICES,
        &CUBE_NORMALS,
        dm.display(),
    ));
    let size = 2;
    let sizef = size as f32;
    let entities: Vec<Entity> = (0..size * size)
        .map(|i| {
            let i = i as f32;
            let mut entity = Entity::new(cube_model.clone());
            entity.scale = 1.0;
            entity.position.set_x((i / sizef).floor() * entity.scale);
            entity.position.set_z((i % sizef) * entity.scale);
            entity
        })
        .collect();

    event_loop.run(move |event, _, control_flow| {
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

        renderer.render(dm.display_mut(), &entities, &camera);
    });
}
