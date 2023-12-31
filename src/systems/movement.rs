use crate::prelude::*;

#[system(for_each)]
#[read_component(Player)]
pub fn movement(
    entity: &Entity,
    want_move: &WantsToMove,
    ecs: &mut SubWorld,
    #[resource] map: &Map,
    #[resource] camera: &mut Camera,
    commands: &mut CommandBuffer
) {
    if map.can_enter_tile(want_move.destination) {
        commands.add_component(want_move.entity, want_move.destination);

        if ecs.entry_ref(want_move.entity).unwrap().get_component::<Player>().is_ok() {
            camera.on_player_move(want_move.destination);
        }
    }

    commands.remove(*entity);
}