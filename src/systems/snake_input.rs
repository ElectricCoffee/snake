use crate::{
    components::{Orientation, SegmentType, Snekment},
    types::History,
    util::position::RichPosition,
};
use amethyst::{
    core::Transform,
    derive::SystemDesc,
    ecs::prelude::*,
    input::{InputHandler, StringBindings},
};

#[derive(SystemDesc)]
pub struct SnakeInput;

impl<'s> System<'s> for SnakeInput {
    type SystemData = (
        WriteStorage<'s, Snekment>,
        ReadStorage<'s, Transform>,
        WriteStorage<'s, Orientation>,
        Read<'s, InputHandler<StringBindings>>,
        Write<'s, History>,
    );

    fn run(
        &mut self,
        (mut snekments, transforms, mut orientations, input, mut history): Self::SystemData,
    ) {
        for (snekment, transform, orientation) in
            (&mut snekments, &transforms, &mut orientations).join()
        {
            if snekment.seg_type == SegmentType::Head {
                let x_dir = input.axis_value("horizontal");
                let y_dir = input.axis_value("vertical");
                if let Some(direction) = get_direction(x_dir, y_dir) {
                    *orientation = direction;
                    let rel_pos = transform.translation().to_relative();
                    history.insert(rel_pos, *orientation);
                }
            }
        }
    }
}

fn get_direction(x: Option<f32>, y: Option<f32>) -> Option<Orientation> {
    use Orientation::*;
    let x = x.unwrap();
    let y = y.unwrap();

    if x > 0. {
        Some(Right)
    } else if x < 0. {
        Some(Left)
    } else if y > 0. {
        Some(Up)
    } else if y < 0. {
        Some(Down)
    } else {
        None
    }
}
