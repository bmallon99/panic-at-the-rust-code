use amethyst::core::{SystemDesc, Transform};
use amethyst::derive::SystemDesc;
use amethyst::ecs::{Join, ReadStorage, System, SystemData, World, WriteStorage};

// You'll have to mark PADDLE_HEIGHT as public in pong.rs
use crate::state::Krab;

#[derive(SystemDesc)]
pub struct KrabSystem;

impl<'s> System<'s> for KrabSystem {
    type SystemData = (WriteStorage<'s, Transform>, ReadStorage<'s, Krab>);

    fn run(&mut self, (mut transforms, krabs): Self::SystemData) {
        for (krab, transform) in (&krabs, &mut transforms).join() {
            let scaled_amount = krab.new_x_position - krab.old_x_position;
            transform.prepend_translation_x(scaled_amount);
        }
    }
}
