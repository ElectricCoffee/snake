use crate::components::Orientation;
use amethyst::{core::*, derive::SystemDesc, ecs::prelude::*};
use std::f32::consts::PI;

#[derive(SystemDesc)]
pub struct SpriteUpdater;

impl<'s> System<'s> for SpriteUpdater {
    type SystemData = (ReadStorage<'s, Orientation>, WriteStorage<'s, Transform>);

    fn run(&mut self, (orientations, mut transforms): Self::SystemData) {
        for (target, transform) in (&orientations, &mut transforms).join() {
            use Orientation::*;

            let angle = match *target {
                Up => PI / 2.,
                Down => 3. * PI / 2.,
                Left => PI,
                Right => 0.,
            };

            transform.set_rotation_2d(angle);
        }
    }
}
