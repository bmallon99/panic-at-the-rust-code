use amethyst::{
    core::timing::Time,
    core::transform::Transform,
    core::SystemDesc,
    derive::SystemDesc,
    ecs::prelude::{Join, Read, ReadStorage, System, SystemData, World, WriteStorage},
};

use crate::state::Crab;

#[derive(SystemDesc)]
pub struct MoveCrabSystem;

impl<'s> System<'s> for MoveCrabSystem {
    type SystemData = (
        ReadStorage<'s, Crab>,
        WriteStorage<'s, Transform>,
        Read<'s, Time>,
    );

    fn run(&mut self, (crabs, mut locals, time): Self::SystemData) {
        // Move the crab according to the time passed since he last jumped
        for (crab, local) in (&crabs, &mut locals).join() {
            let d_time = time.absolute_time_seconds()-crab.jump_start_time;
            let a = -1.5;
            local.prepend_translation_y((crab.velocity * d_time + 0.5*(a)*d_time.powf(2.0)) as f32);
        }
    }
}