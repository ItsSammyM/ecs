use crate::ecs::{component::components::Components, system::{condition_system_container::{ConditionSystemContainerTrait, IntoConditionSystem}, system_container::{IntoSystem, SystemContainerTrait}}};

pub(crate) struct Systems(Vec<ConditionalSystem>);
impl Systems{
    pub(crate) fn new()->Self{
        Self(Vec::new())
    }
}
struct ConditionalSystem{
    condition: Box<dyn ConditionSystemContainerTrait>,
    system: Box<dyn SystemContainerTrait>
}
impl ConditionalSystem{
    pub(crate) fn run(&mut self, mut components: &mut Components) {
        if self.condition.run(&mut components) {
            self.system.run(&mut components);
        }
    }
}

impl Systems{
    pub(crate) fn add_conditional_system<
        SInput, CInput, 
        C: ConditionSystemContainerTrait + 'static,
        S: SystemContainerTrait + 'static
    >(
        &mut self,
        condition: impl IntoConditionSystem<CInput, System = C>,
        system: impl IntoSystem<SInput, System = S>
    ) {
        self.0.push(ConditionalSystem{
            condition: Box::new(condition.into_condition()),
            system: Box::new(system.into_system())
        });
    }
    pub(crate) fn add_system<
        Input,
        S: SystemContainerTrait + 'static
    >(
        &mut self,
        system: impl IntoSystem<Input, System = S>
    ) {
        self.add_conditional_system(||true, system);
    }
    pub(crate) fn run(&mut self, components: &mut Components){
        self.0.iter_mut().for_each(|s|s.run(components));
    }
}