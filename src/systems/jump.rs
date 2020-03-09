use amethyst::{
    core::{Transform, SystemDesc},
    core::timing::Time,
    derive::SystemDesc,
    ecs::prelude::{Join, Read, ReadStorage, System, SystemData, World, WriteStorage},
};

use crate::state::{Crab, Platform};

#[derive(SystemDesc)]
pub struct JumpSystem;

impl<'s> System<'s> for JumpSystem {
    type SystemData = (
        WriteStorage<'s, Crab>,
        ReadStorage<'s, Platform>,
        ReadStorage<'s, Transform>,
        Read<'s, Time>,
    );

    fn run(&mut self, (mut crabs, platforms, transforms, time): Self::SystemData) {
        // Check whether the crab collided with plaform, and jump accordingly.
        //
        // We also check for the velocity of the crab every time, to prevent multiple collisions
        // from occurring.
        for (crab, transform) in (&mut crabs, &transforms).join() {
            let crab_x = transform.translation().x;
            let crab_y = transform.translation().y;

            // Bounce at the top or the bottom of the arena.
            if crab_y <= 0.0
            {
                // game overrrrr
                println!("you suck");
            }

            // Bounce at the paddles.
            for (platform, platform_transform) in (&platforms, &transforms).join() {
                let platform_x = platform_transform.translation().x - (platform.width * 0.5);
                let platform_y = platform_transform.translation().y - (platform.height * 0.5);

                // To determine whether the ball has collided with a paddle, we create a larger
                // rectangle around the current one, by subtracting the ball radius from the
                // lowest coordinates, and adding the ball radius to the highest ones. The ball
                // is then within the paddle if its center is within the larger wrapper
                // rectangle.
                if point_in_rect(
                    crab_x,
                    crab_y,
                    platform_x - crab.width/2.0,
                    platform_y + platform.height + crab.height/2.0 - 10.0,
                    platform_x + platform.width + crab.width/2.0,
                    platform_y + platform.height + crab.height/2.0,
                ) {
                    // jump again
                    crab.jump_start_time = time.absolute_time_seconds();
                }
            }
        }
    }
}

// A point is in a box when its coordinates are smaller or equal than the top
// right and larger or equal than the bottom left.
fn point_in_rect(x: f32, y: f32, left: f32, bottom: f32, right: f32, top: f32) -> bool {
    x >= left && x <= right && y >= bottom && y <= top // need to change so you can go through bottom of platforms
}