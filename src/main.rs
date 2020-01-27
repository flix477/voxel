mod entities;
mod input;
mod math;
mod models;
mod rendering;

use entities::{camera::Camera, Entity, Updatable};
use glium::glutin::EventsLoop;
use input::Input;
use models::primitives::{QUAD_INDICES, QUAD_VERTICES};
use rendering::{display::DisplayManager, loader::Loader, renderer::Renderer};

fn main() {
    run();
}

fn run() {
    let mut events_loop = EventsLoop::new();
    let mut dm = DisplayManager::new(&events_loop).unwrap();
    let mut renderer = Renderer::new(dm.display());
    let mut input = Input::default();
    let mut camera = Camera::default();

    let model = Loader::create_model(&QUAD_VERTICES, &QUAD_INDICES, dm.display());
    let mut entity = Entity::new(model);

    while !dm.should_close() {
        events_loop.poll_events(|event| {
            dm.on_event(&event);
            input.on_event(&event);
        });

        camera.update(&input);
        entity.position.set_z(entity.position.z() - 0.001);
        dbg!(entity.position.z());

        renderer.render(dm.display_mut(), &entity, &camera);
    }
}
