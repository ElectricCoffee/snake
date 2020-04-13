use crate::components::{Orientation, SegmentType, Snekment};
use amethyst::{
    derive::SystemDesc,
    ecs::prelude::*,
    input::{InputHandler, StringBindings},
};

#[derive(SystemDesc)]
pub struct SnakeInput;

impl<'s> System<'s> for SnakeInput {
    type SystemData = (
        WriteStorage<'s, Snekment>,
        Read<'s, InputHandler<StringBindings>>,
    );

    fn run(&mut self, (mut snekments, input): Self::SystemData) {
        for snekment in (&mut snekments).join() {
            if snekment.seg_type == SegmentType::Head {
                let x_dir = input.axis_value("horizontal");
                let y_dir = input.axis_value("vertical");
                if let Some(direction) = get_direction(x_dir, y_dir) {
                    snekment.orientation = direction;
                }
            }
        }
    }
}

fn get_direction(x: Option<f32>, y: Option<f32>) -> Option<Orientation> {
    let x = x.unwrap();
    let y = y.unwrap();

    if x > 0. {
        Some(Orientation::Right)
    } else if x < 0. {
        Some(Orientation::Left)
    } else if y > 0. {
        Some(Orientation::Up)
    } else if y < 0. {
        Some(Orientation::Down)
    } else {
        None
    }
}
