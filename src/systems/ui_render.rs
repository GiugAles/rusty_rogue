use crate::prelude::*;

#[system]
pub fn ui_render(#[resource] camera: &Camera, #[resource] monster_count: &usize) {
    let mut draw_batch = DrawBatch::new();
    draw_batch.target(2);

    if *monster_count > 9 {
        draw_batch.set(
            Point::new(1, 1),
            ColorPair::new(WHITE, BLACK),
            to_cp437('>')
        );
    } else {
        draw_batch.set(
            Point::new(1, 1),
            ColorPair::new(WHITE, BLACK),
            to_cp437(char::from_digit(*monster_count as u32, 10).unwrap())
        );
    }
    draw_batch.submit(5000).expect("Batch error");
}