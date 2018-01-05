extern crate amethyst;

use amethyst::renderer::{Stage, Pipeline, PosTex, RenderBundle, RenderSystem, DisplayConfig};
use amethyst::prelude::*;
use amethyst::core::transform::TransformBundle;

mod states;

use states::playstate::play::PlayState;


fn run() -> Result<(), amethyst::Error> {

    let resources = format!("{}/resources", env!("CARGO_MANIFEST_DIR"));

    let mut game = Application::build(resources, PlayState)?;

    Ok(game.build()?.run())
}

fn main() {
    if let Err(e) = run() {
        eprintln!("Failed to execute example: {}", e);
        ::std::process::exit(1);
    }
}
