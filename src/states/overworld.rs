use amethyst::{
    assets::{AssetStorage, Loader, Handle},
    core::transform::Transform,
    prelude::*,
    renderer::{Camera, ImageFormat, SpriteRender, SpriteSheet, SpriteSheetFormat, Texture}
};

use crate::components::Trainer;

pub const SCREEN_HEIGHT: f32 = 500.0;
pub const SCREEN_WIDTH: f32 = 500.0;

/// Load the overworld spritesheet into the world.
fn load_sprite_sheet(world: &mut World) -> Handle<SpriteSheet> {
    let texture_handle = {
        let loader = world.read_resource::<Loader>();
        let texture_storage = world.read_resource::<AssetStorage<Texture>>();
        loader.load(
            "texture/gold_trainer_spritesheet.png",
            ImageFormat::default(),
            (),
            &texture_storage,
        )
    };

    let loader = world.read_resource::<Loader>();
    let sprite_sheet_store = world.read_resource::<AssetStorage<SpriteSheet>>();
    loader.load(
        "texture/gold_trainer_spritesheet.ron",
        SpriteSheetFormat(texture_handle),
        (),
        &sprite_sheet_store,
    )
}

// Setup and create the default trainer
fn initialize_trainer(world: &mut World, sprite_sheet_handle: Handle<SpriteSheet>) {
    let mut transform = Transform::default();

    let x = SCREEN_WIDTH / 2.0;
    let y = SCREEN_HEIGHT / 2.0;
    let z = 0.0;

    transform.set_translation_xyz(x, y, z);

    let sprite_render = SpriteRender {
        sprite_sheet: sprite_sheet_handle.clone(),
        sprite_number: 0,
    };

    println!();

    world
        .create_entity()
        .with(Trainer::new())
        .with(transform)
        .with(sprite_render.clone())
        .build();
}

// Setup camera in a way that our screen covers whole arena and (0, 0) is in the bottom left.
fn initialize_camera(world: &mut World) {
    let mut transform = Transform::default();
    let x = SCREEN_WIDTH * 0.5;
    let y = SCREEN_HEIGHT * 0.5;
    let z = 1.0;
    transform.set_translation_xyz(x, y, z);

    world
        .create_entity()
        .with(Camera::standard_2d(SCREEN_WIDTH, SCREEN_HEIGHT))
        .with(transform)
        .build();
}

#[derive(Default)]
pub struct Overworld {
    sprite_sheet_handle: Option<Handle<SpriteSheet>>,
}

impl SimpleState for Overworld {
    fn on_start(&mut self, _data: StateData<'_, GameData<'_, '_>>) {
        let world = _data.world;

        self.sprite_sheet_handle.replace(load_sprite_sheet(world));

        world.register::<Trainer>();

        initialize_trainer(world, self.sprite_sheet_handle.clone().unwrap());
        initialize_camera(world);
    }
}
