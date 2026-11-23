use super::system::condition_system_container::IntoConditionSystem;
use super::system::system_container::IntoSystem;
use super::{component::components::Components, system::systems::Systems};
use crate::ecs::system::condition_system_container::ConditionSystemContainerTrait;
use crate::ecs::system::system_container::SystemContainerTrait;
use crate::prelude::*;

pub struct SchedulerBuilder {
    inner: Scheduler
}
impl SchedulerBuilder {
    pub fn new()->Self{
        Self { inner: 
            Scheduler{
                systems: Systems::new(),
                components: Components::new(),
            }
        }
    }
    pub fn add_system<Input, S: SystemContainerTrait + 'static>(mut self, system: impl IntoSystem<Input, System = S>)->Self{
        self.inner.systems.add_system(system);
        self
    }
    pub fn add_conditional_system<
        SInput, CInput, 
        C: ConditionSystemContainerTrait + 'static,
        S: SystemContainerTrait + 'static
    >(
        mut self,
        condition: impl IntoConditionSystem<CInput, System = C>,
        system: impl IntoSystem<SInput, System = S>
    ) -> Self {
        self.inner.systems.add_conditional_system(condition, system);
        self
    }
    pub fn add_resource<C: Component + 'static>(mut self, component: C) -> Self {
        self.inner.components.add_resource(component);
        self
    }
    pub fn add_plugin<P: Plugin>(self, plugin: P)->Self{
        plugin.add_plugin(self)
    }
    pub fn build_scheduler(self)->Scheduler{
        self
            .add_resource(Commands::new())
            .inner
    }
}