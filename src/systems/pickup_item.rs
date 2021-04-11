use crate::prelude::*;

#[system(for_each)]
#[read_component(WantsToPickup)]
#[read_component(Point)]
#[read_component(Item)]
#[read_component(Weapon)]
#[read_component(Carried)]
pub fn pickup_item(
    entity: &Entity,
    wants_to_pickup: &WantsToPickup,
    ecs: &mut SubWorld,
    commands: &mut CommandBuffer,
) {
    if let Ok(entry) = ecs.entry_ref(wants_to_pickup.who) {
        if let Ok(item_pos) = entry.get_component::<Point>() {
            <(Entity, &Item, &Point)>::query()
                .iter(ecs)
                .filter(|(_, _, pos)| *pos == item_pos)
                .for_each(|(e, i, _)| {
                    commands.remove_component::<Point>(*e);
                    commands.add_component(*e, Carried(wants_to_pickup.who));
                    if let Ok(item) = ecs.entry_ref(*e) {
                        if item.get_component::<Weapon>().is_ok() {
                            <(Entity, &Carried, &Weapon)>::query()
                                .iter(ecs)
                                .filter(|(_, c, _)| c.0 == wants_to_pickup.who)
                                .for_each(|(e, c, w)| commands.remove(*e))
                        }
                    }
                });
        }
    }
    commands.remove(*entity);
}
