use amethyst::{
    core::SystemDesc,
    derive::SystemDesc,
    ecs::prelude::{Join, ReadStorage, Read, System, SystemData, World, WriteStorage},
    input::{InputHandler, StringBindings},
    renderer::SpriteRender
};

use crate::components::Trainer;

#[derive(Debug, PartialEq)]
enum Direction {
    Down,
    Left,
    Right,
    Up,
}

static SPRITES_PER_DIRECTION: usize = 3;

static DOWN_MIN: usize = 0;
static DOWN_MAX: usize = DOWN_MIN + SPRITES_PER_DIRECTION;

static LEFT_MIN: usize = 4;
static LEFT_MAX: usize = LEFT_MIN + SPRITES_PER_DIRECTION;

static RIGHT_MIN: usize = 8;
static RIGHT_MAX: usize = RIGHT_MIN + SPRITES_PER_DIRECTION;

static UP_MIN: usize = 12;
static UP_MAX: usize = UP_MIN + SPRITES_PER_DIRECTION;

#[derive(SystemDesc)]
pub struct TrainerMovementSystem;

impl<'s> System<'s> for TrainerMovementSystem {
    type SystemData = (
        ReadStorage<'s, Trainer>,
        WriteStorage<'s, SpriteRender>,
        Read<'s, InputHandler<StringBindings>>
    );

    fn run(&mut self, (trainers, mut sprite_renders, input): Self::SystemData) {
        for (_, sprite_render) in (&trainers, &mut sprite_renders).join() {
            // Determine the new direction based on inputs
            if let Some(new_direction) = get_input_direction(&input) {
                // Get the current direction
                let original_direction = get_sprite_direction(sprite_render.sprite_number);

                // Get the iteration count (is it the 1st/2nd/3rd/4th frame of the animation)
                let (original_min, original_max) = get_sprite_range(&original_direction);

                // Only inclusive on the minimum end because we can't increment past the max
                let sprite_in_range = original_min <= sprite_render.sprite_number && sprite_render.sprite_number < original_max;

                // If the new direction is the same as the original AND the sprite number hasn't
                // overflown the directional max yet, increment the frame
                if new_direction == original_direction && sprite_in_range {
                    sprite_render.sprite_number += 1;
                } else {
                    let (new_min, _) = get_sprite_range(&new_direction);
                    sprite_render.sprite_number = new_min
                }
            }
        }
    }
}

fn get_sprite_range(direction: &Direction) -> (usize, usize) {
    let value = match direction {
        Direction::Down => (DOWN_MIN, DOWN_MAX),
        Direction::Left => (LEFT_MIN, LEFT_MAX),
        Direction::Right => (RIGHT_MIN, RIGHT_MAX),
        Direction::Up => (UP_MIN, UP_MAX),
    };

    return value;
}

fn get_sprite_direction(sprite_number: usize) -> Direction {
    let mut direction = Direction::Down;

    if LEFT_MIN <= sprite_number && sprite_number <= LEFT_MAX {
        direction = Direction::Left;
    }

    if RIGHT_MIN <= sprite_number && sprite_number <= RIGHT_MAX {
        direction = Direction::Right;
    }

    if UP_MIN <= sprite_number && sprite_number <= UP_MAX {
        direction = Direction::Up;
    }

    return direction;
}

fn get_input_direction(input: &Read<InputHandler<StringBindings>>) -> Option<Direction> {
    let mut direction = None;

    if let Some(value) = input.axis_value("horizontal") {
        if value < 0.0 {
            direction = Some(Direction::Left);
        }
        if value > 0.0 {
            direction = Some(Direction::Right);
        }
    }

    if let Some(value) = input.axis_value("vertical") {
        if value > 0.0 {
            direction = Some(Direction::Up);
        }
        if value < 0.0 {
            direction = Some(Direction::Down);
        }
    }

    return direction;
}
