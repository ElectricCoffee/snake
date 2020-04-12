use crate::components::Snekment;
use amethyst::core::Transform;
use amethyst::ecs::prelude::*;

//#[derive(SystemDesc)]
pub struct Movement;

impl<'s> System<'s> for Movement {
    type SystemData = (
        ReadStorage<'s, Snekment>,
        WriteStorage<'s, Transform>,
        Entities<'s>,
    );

    fn run(&mut self, (snekments, mut transforms, entities): Self::SystemData) {
        for (snekment, transform) in (&snekments, &mut transforms).join() {}
    }
}
