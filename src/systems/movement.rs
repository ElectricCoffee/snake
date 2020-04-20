use crate::{
    components::{Orientation, SegmentType, Snekment},
    types::History,
    util::position::{Relative, RichPosition},
};
use amethyst::{core::Transform, derive::SystemDesc, ecs::prelude::*};

#[derive(SystemDesc)]
pub struct Movement;

impl<'s> System<'s> for Movement {
    type SystemData = (
        WriteStorage<'s, Snekment>,
        WriteStorage<'s, Transform>,
        WriteStorage<'s, Orientation>,
        Write<'s, History>,
    );

    fn run(
        &mut self,
        (mut snekments, mut transforms, mut orientations, mut history): Self::SystemData,
    ) {
        for (snekment, transform, orientation) in
            (&mut snekments, &mut transforms, &mut orientations).join()
        {
            let mut relative_pos = transform.translation().to_relative();

            // first move
            relative_pos += match orientation {
                Orientation::Up => Relative::up(),
                Orientation::Down => Relative::down(),
                Orientation::Left => Relative::left(),
                Orientation::Right => Relative::right(),
            };

            // then check if position is in the history
            if history.contains_key(&relative_pos) {
                match snekment.seg_type {
                    // If a head finds a position that already exists, set the orientation to the head's
                    SegmentType::Head => {
                        history.insert(relative_pos, *orientation);
                    }
                    // If a body segment finds the coordinate, switch to the stored orientation
                    SegmentType::Body => {
                        *orientation = history[&relative_pos];
                    }
                    // Technically not necessary, but if a tail segment passes over a coordinate, it'll be removed from the list
                    // This is unnecessary because of what the Head does. Technically only either the head or tail logics are required.
                    SegmentType::Tail => {
                        *orientation = history[&relative_pos];
                        history.remove(&relative_pos);
                    }
                }
            }
            // commit the movement
            let abs_pos = relative_pos.to_absolute();
            transform.set_translation_xyz(abs_pos.0, abs_pos.1, 0.);
        }
    }
}
