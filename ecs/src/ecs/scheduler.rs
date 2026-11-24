use std::{any::{Any, TypeId}, cell::RefCell, collections::HashSet, time::{Duration, Instant}};
use ecs_macros::ComponentInCrate;

use crate::ecs::{
    bundle::Bundle,
    component::{component::Component, components::Components},
    entity::EntityId,
    system::systems::Systems
};


pub struct Scheduler {
    pub(crate) systems: Systems,
    pub(crate) components: Components,
}

pub enum SchedulerTargetFramerate{
    Target(f32),
    Max
}
impl From<f32> for SchedulerTargetFramerate{
    fn from(value: f32) -> Self {
        if value <= 0.0 {
            SchedulerTargetFramerate::Max
        }else{
            SchedulerTargetFramerate::Target(value)
        }
    }
}

impl Scheduler {
    pub fn add_resource<C: Component + 'static>(&mut self, component: C) {
        self.components.add_resource(component);
    }
    pub fn run(&mut self) {
        self.systems.run(&mut self.components);
        Commands::after_run(&mut self.components);
    }
    /// # Panics 
    /// panic if target_framerate <= 0
    pub fn run_blocking(mut self, target_framerate: impl Into<SchedulerTargetFramerate>) {
        match target_framerate.into() {
            SchedulerTargetFramerate::Target(fps) => {
                let mut next_frame = Instant::now();
                let desired_dt = Duration::from_secs_f32(1.0/fps);
                loop {
                    if Instant::now() < next_frame {continue}
                    next_frame = Instant::now() + desired_dt;
                    self.run();
                }
            },
            SchedulerTargetFramerate::Max => {
                loop {
                    self.run();
                }
            },
        };

    }
}



////

#[derive(ComponentInCrate)]
pub struct Commands{
    greatest_entity: EntityId,
    entities_to_spawn: Components,
    entities_to_delete: HashSet<EntityId>
}
impl Commands {
    pub(crate) fn new() -> Self{
        Self { greatest_entity: EntityId::new(0), entities_to_spawn: Components::new(), entities_to_delete: HashSet::new() }
    }
    fn drain_spawn(&mut self) -> impl Iterator<Item = ((EntityId, TypeId), RefCell<Box<dyn Any>>)> {
        self.entities_to_spawn.drain()
    }
    fn drain_delete(&mut self) -> impl Iterator<Item = EntityId> {
        self.entities_to_delete.drain()
    }
    fn after_run(components: &mut Components) {
        let commands_key = (EntityId::new(0), TypeId::of::<Commands>());

        let mut commands = unsafe{
            components.remove_component_unchecked::<Commands>(&commands_key)
        };

        commands
            .drain_delete()
            .for_each(|e|components.remove_entity(e));

        commands
            .drain_spawn()
            .for_each(|(k, v)|{
                components.0.insert(k, v);
            });
        
        components.add_component(commands_key.0, commands);
    }
    pub fn new_entity(&mut self) -> EntitySpawnBuilder<'_> {
        let new_id = EntityId::new(self.greatest_entity.id() + 1);
        self.greatest_entity = new_id.clone();
        EntitySpawnBuilder{
            id: new_id.clone(),
            commands: self
        }.add_component(new_id)
    }
    pub fn delete_entity(&mut self, entity: EntityId) {
        self.entities_to_delete.insert(entity);
    }
}
pub struct EntitySpawnBuilder<'a>{
    id: EntityId,
    commands: &'a mut Commands
}
impl<'a> EntitySpawnBuilder<'a>{
    pub fn add_component<C: Component + 'static>(self, component: C) -> Self {
        self.commands.entities_to_spawn.add_component(self.id, component);
        self
    }
    pub fn add_bundle<B: Bundle + 'static>(self, bundle: B) -> Self {
        bundle.add_bundle(self)
    }
    pub fn entity_id(&self) -> EntityId {
        self.id
    }
}