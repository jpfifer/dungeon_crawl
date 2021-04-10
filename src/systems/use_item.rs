use crate::prelude::*;

#[system]
#[read_component(ActivateItem)]
#[read_component(ProvidesHealing)]
#[read_component(ProvidesDungeonMap)]
#[write_component(Health)]
pub fn use_item(ecs: &mut SubWorld, commands: &mut CommandBuffer, #[resource] map: &mut Map) {
    let mut healing_to_apply = Vec::<(Entity, i32)>::new();
    <(Entity, &ActivateItem)>::query()
        .iter(ecs)
        .for_each(|(entity, activate)| {
            let item = ecs.entry_ref(activate.item);
            if let Ok(item) = item {
                if let Ok(healing) = item.get_component::<ProvidesHealing>() {
                    healing_to_apply.push((activate.used_by, healing.amount));
                }
                if item.get_component::<ProvidesDungeonMap>().is_ok() {
                    map.revealed_tiles.iter_mut().for_each(|t| *t = true);
                }
            }
            commands.remove(activate.item);
        });

    healing_to_apply.iter().for_each(|(entity, add)| {
        if let Ok(mut entry) = ecs.entry_mut(*entity) {
            if let Ok(health) = entry.get_component_mut::<Health>() {
                health.current = i32::min(health.max, health.current + add)
            }
        }
    })
}
