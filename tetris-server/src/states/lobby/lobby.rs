use amethyst::winit::{Event, KeyboardInput, VirtualKeyCode, WindowEvent};
use amethyst::ecs::World;
use amethyst::core::transform::{LocalTransform, Transform};
use amethyst::assets::Loader;
use amethyst::core::cgmath::Vector3;

use amethyst::prelude::*;

pub struct LobbyState;

impl State for LobbyState {
    fn on_start(&mut self, world: &mut World) {

    }

}