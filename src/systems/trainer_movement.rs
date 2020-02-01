use amethyst::{
    core::{SystemDesc, transform::Transform},
    derive::SystemDesc,
    ecs::prelude::{Join, ReadStorage, Read, System, SystemData, World, WriteStorage},
    input::{InputHandler, StringBindings},
    renderer::SpriteRender
};

use crate::components::Trainer;

#[derive(SystemDesc)]
pub struct TrainerMovementSystem;

impl<'s> System<'s> for TrainerMovementSystem {
    type SystemData = (
        ReadStorage<'s, Trainer>,
        WriteStorage<'s, SpriteRender>,
        Read<'s, InputHandler<StringBindings>>
    );

    fn run(&mut self, (trainers, mut sprite_renders, input): Self::SystemData) {
        for (trainer, sprite_render) in (&trainers, &mut sprite_renders).join() {
            if let Some(value) = input.axis_value("vertical") {
                if value > 0.0 {
                    println!("Up: {}", value);
                    sprite_render.sprite_number = 12
                }
                if value < 0.0 {
                    println!("Down: {}", value);
                    sprite_render.sprite_number = 0
                }
            }
            if let Some(value) = input.axis_value("horizontal") {
                if value > 0.0 {
                    println!("Right: {}", value);
                    sprite_render.sprite_number = 8
                }
                if value < 0.0 {
                    println!("Left: {}", value);
                    sprite_render.sprite_number = 4
                }
            }
        }
    }
}
