use std::{any::{Any, TypeId}, cell::{RefCell, RefMut}, collections::HashMap};
use crate::prelude::*;

pub struct Components(pub(crate) HashMap<(EntityId, TypeId), RefCell<Box<dyn Any>>>);
impl Components{
    pub(crate) fn new()->Self{
        Self(HashMap::new())
    }
    pub(crate) fn iter(&self) -> impl Iterator<Item = (&(EntityId, TypeId), &RefCell<Box<dyn Any>>)> {
        self.0.iter()
    }
    pub(crate) fn get(&self, key: &(EntityId, TypeId)) -> Option<&RefCell<Box<dyn Any>>>{
        self.0.get(key)
    }
    pub(crate) fn get_component<C: Component + 'static>(&self, entity: EntityId) -> Option<RefMut<'_, C>>{
        self
            .get(&(entity, TypeId::of::<C>()))
            .map(|r| RefMut::map(r.borrow_mut(), |r: &mut Box<dyn Any + 'static>|r.downcast_mut::<C>().unwrap()))
    }
    pub(crate) fn drain(&mut self) -> impl Iterator<Item = ((EntityId, TypeId), RefCell<Box<dyn Any>>)> {
        self.0.drain()
    }
    pub(crate) unsafe fn remove_component_unchecked<T: Component + 'static>(&mut self, key: &(EntityId, TypeId)) -> T {
        *self.0.remove(key).unwrap().into_inner().downcast::<T>().unwrap()
    }


    pub(crate) fn add_resource<C: Component + 'static>(&mut self, component: C) {
        self.add_component(EntityId::new(0), component);
    }
    pub(crate) fn add_component<C: Component + 'static>(&mut self, entity: EntityId, component: C) {
        self.0
            .insert(
                (entity, TypeId::of::<C>()),
                RefCell::new(Box::new(component))
            );
    }
    pub(crate) fn remove_component<C: Component + 'static>(&mut self, entity: EntityId) -> Option<RefCell<Box<dyn Any + 'static>>>{
        self.0.remove(&(entity, TypeId::of::<C>()))
    }
    pub(crate) fn remove_entity(&mut self, entity: EntityId) {
        self.0.retain(|(e, _), _|*e != entity);
    }
}