use crate::{
    components::{Orientation, SegmentType, Snekment},
    util::position as pos,
    util::position::RichPosition,
};
use amethyst::{core::Transform, derive::SystemDesc, ecs::prelude::*};
use std::collections::HashMap;

#[derive(SystemDesc)]
pub struct Movement {
    history: HashMap<pos::Relative, Orientation>,
}

impl Default for Movement {
    fn default() -> Self {
        Movement {
            history: HashMap::new(),
        }
    }
}

impl<'s> System<'s> for Movement {
    type SystemData = (WriteStorage<'s, Snekment>, WriteStorage<'s, Transform>);

    fn run(&mut self, (mut snekments, mut transforms): Self::SystemData) {
        for (snekment, transform) in (&mut snekments, &mut transforms).join() {
            let mut relative_pos = transform.translation().to_relative();

            // first move
            match snekment.orientation {
                Orientation::Up => relative_pos.1 += 1,
                Orientation::Down => relative_pos.1 -= 1,
                Orientation::Left => relative_pos.0 -= 1,
                Orientation::Right => relative_pos.0 += 1,
            }
            // then check if position is in the history
            if self.history.contains_key(&relative_pos) {
                match snekment.seg_type {
                    // If a head finds a position that already exists, set the orientation to the head's
                    SegmentType::Head => {
                        self.history.insert(relative_pos, snekment.orientation);
                    }
                    // If a body segment finds the coordinate, switch to the stored orientation
                    SegmentType::Body => {
                        snekment.orientation = self.history[&relative_pos];
                    }
                    // Technically not necessary, but if a tail segment passes over a coordinate, it'll be removed from the list
                    // This is unnecessary because of what the Head does. Technically only either the head or tail logics are required.
                    SegmentType::Tail => {
                        self.history.remove(&relative_pos);
                    }
                }
            }
            // commit the movement
            let abs_pos = relative_pos.to_absolute();
            transform.set_translation_xyz(abs_pos.0, abs_pos.1, 0.);
        }
    }
}
