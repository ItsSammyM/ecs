use std::{any::TypeId, cell::RefCell};

use crate::{ecs::component::components::Components, prelude::*};

#[allow(private_bounds)]
pub trait Component: ComponentGetBoxedAny{}


pub(crate) trait ComponentGetBoxedAny: 'static {
    fn get_boxed_any<'a>(entity: EntityId, components: &Components) -> Option<&RefCell<Box<dyn std::any::Any>>>;
}
impl<C: Component + 'static> ComponentGetBoxedAny for C {
    fn get_boxed_any<'a>(entity: EntityId, components: &Components) -> Option<&RefCell<Box<dyn std::any::Any>>> {
        components.get(&(entity, TypeId::of::<Self>()))
    }
}