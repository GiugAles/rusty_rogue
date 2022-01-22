mod map_render;
mod player_input;
mod entity_render;
mod collisions;
mod ui_render;

use crate::prelude::*;

pub fn build_scheduler() -> Schedule {
    Schedule::builder()
        .add_system(player_input::player_input_system())
        .add_system(collisions::collision_system())
        .add_system(map_render::map_render_system())
        .add_system(entity_render::entity_render_system())
        .add_system(ui_render::ui_render_system())
        .build()
}