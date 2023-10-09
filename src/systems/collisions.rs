use crate::prelude::*;

#[system]
#[read_component(Point)]
#[read_component(Player)]
#[read_component(Ennemy)]
pub fn collisions(ecs: &SubWorld, commands: &mut CommandBuffer) {
    let mut player_pos = Point::zero();

    <&Point>::query()
        .filter(component::<Player>())
        .iter(ecs)
        .for_each(|pos| player_pos = *pos);

    let mut ennemies = <(Entity, &Point)>::query()
        .filter(component::<Ennemy>());

    ennemies
        .iter(ecs)
        .filter(|(_, pos)| **pos == player_pos)
        .for_each(|(entity, _)| commands.remove(*entity));
}
