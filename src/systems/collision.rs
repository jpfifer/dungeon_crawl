use crate::prelude::*;

#[system]
#[read_component(Point)]
pub fn collisions(ecs: &mut SubWorld, commands: &mut CommandBuffer) {}
