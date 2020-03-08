use amethyst::{
    assets::{AssetStorage, Loader, Handle},
    core::{ArcThreadPool, transform::Transform, Time},
    ecs::prelude::Entity,
    input::{Bindings, InputSystemDesc, StringBindings},
    prelude::*,
    renderer::{Camera, ImageFormat, SpriteRender, SpriteSheet, SpriteSheetFormat, Texture},
    shred::{Dispatcher, DispatcherBuilder},
    ui::{UiCreator, UiFinder, UiText},
    utils::{fps_counter::FpsCounter, application_root_dir},
};

use crate::components::Trainer;
use crate::systems::TrainerMovementSystem;

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
pub struct Overworld<'a, 'b> {
    sprite_sheet_handle: Option<Handle<SpriteSheet>>,
    fps_display: Option<Entity>,
    dispatcher: Option<Dispatcher<'a, 'b>>,
}

impl<'a, 'b> SimpleState for Overworld<'a, 'b> {
    fn on_start(&mut self, _data: StateData<'_, GameData<'_, '_>>) {
        let world = _data.world;

        let mut dispatcher_builder = DispatcherBuilder::new();

        // Hacky workaround to explicitly add our input_system bindings to this dispatcher
        dispatcher_builder.add(
            InputSystemDesc::<StringBindings>::new(self.get_bindings()).build(world),
            "input_system",
            &[]
        );

        dispatcher_builder.add(TrainerMovementSystem, "trainer_movement_system", &["input_system"]);

        let mut dispatcher = dispatcher_builder
            .with_pool((*world.read_resource::<ArcThreadPool>()).clone())
            .build();

        dispatcher.setup(world);

        self.dispatcher = Some(dispatcher);

        self.sprite_sheet_handle.replace(load_sprite_sheet(world));

        initialize_trainer(world, self.sprite_sheet_handle.clone().unwrap());
        initialize_camera(world);

        world.exec(|mut creator: UiCreator<'_>| {
            creator.create("ui/overworld.ron", ());
        });
    }

    fn update(&mut self, _data: &mut StateData<'_, GameData<'_, '_>>) -> SimpleTrans {
        let StateData { world, .. } = _data;

        if let Some(dispatcher) = self.dispatcher.as_mut() {
            dispatcher.dispatch(world);
        }

        self.update_fps(world);

        Trans::None
    }
}

impl<'a, 'b> Overworld<'a, 'b> {

    // Update the FPS counter
    fn update_fps(&mut self, world: &mut World) {
        if self.fps_display.is_none() {
            world.exec(|finder: UiFinder<'_>| {
                if let Some(entity) = finder.find("fps") {
                    self.fps_display = Some(entity);
                }
            });
        }

        let mut ui_text = world.write_storage::<UiText>();
        {
            if let Some(fps_display) = self.fps_display.and_then(|entity| ui_text.get_mut(entity)) {
                if world.read_resource::<Time>().frame_number() % 20 == 0 {
                    let fps = world.read_resource::<FpsCounter>().sampled_fps();
                    fps_display.text = format!("FPS: {:.*}", 2, fps);
                }
            }
        }
    }

    fn get_bindings(&mut self) -> Option<Bindings<StringBindings>> {
        let app_root = application_root_dir().unwrap();
        let config_root = app_root.join("config");
        let mut bindings = Bindings::load(config_root.join("bindings.ron"));
        match bindings.check_invariants() {
            Ok(_) => {},
            Err(_) => println!("Invalid bindings!"),
        }
        Some(bindings)
    }
}
