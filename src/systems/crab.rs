use amethyst::core::{SystemDesc, Transform};
use amethyst::derive::SystemDesc;
use amethyst::ecs::{Join, Read, ReadStorage, System, SystemData, World, WriteStorage};
use amethyst::input::{InputHandler, StringBindings};

// You'll have to mark PADDLE_HEIGHT as public in pong.rs
use crate::state::Crab;

#[derive(SystemDesc)]
pub struct CrabSystem;

impl<'s> System<'s> for CrabSystem {
    type SystemData = (
        WriteStorage<'s, Transform>,
        ReadStorage<'s, Crab>,
        Read<'s, InputHandler<StringBindings>>,
    );

    fn run(&mut self, (mut transforms, crabs, input): Self::SystemData) {
        for (_crab, transform) in (&crabs, &mut transforms).join() {
            if let Some(mv_amount) = input.axis_value("crab") {
                let scaled_amount = 10. * mv_amount as f32;
                transform.prepend_translation_x(scaled_amount);
            }
        }
    }
}
