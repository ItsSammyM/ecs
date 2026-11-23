use crate::{ecs::system::system_container::SystemContainerTrait, prelude::*};
use ecs_macros::ComponentInCrate;

/// Ensure you add this plugin after you add plugins that depend on it. Its system needs to be after systems that depend on it
pub struct FirstFramePlugin;
impl Plugin for FirstFramePlugin {
    fn add_plugin(self, builder: SchedulerBuilder)->SchedulerBuilder {
        builder
            .add_resource(IsFirstFrame(true))
            .add_start_system(|mut r: ResMut<IsFirstFrame>|r.0 = false)
    }
}

#[derive(ComponentInCrate)]
pub struct IsFirstFrame(bool);
impl IsFirstFrame{
    pub fn is_first_frame(&self)->bool{
        self.0
    }
}
impl SchedulerBuilder{
    pub fn add_start_system<Input, S: SystemContainerTrait + 'static>(self, system: impl IntoSystem<Input, System = S>)->Self{
        self.add_conditional_system(|r: Res<IsFirstFrame>|r.is_first_frame(), system)
    }
}