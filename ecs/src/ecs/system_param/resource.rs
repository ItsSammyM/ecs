use std::{any::{Any, TypeId}, cell::{Ref, RefMut}, marker::PhantomData, ops::{Deref, DerefMut}};

use crate::ecs::{entity::EntityId, component::components::Components, system_param::system_param::SystemParameter};

pub struct Res<'a, T: 'static> {
    value: Ref<'a, Box<dyn Any>>,
    _marker: PhantomData<&'a T>,
}
impl<'res, T: 'static> SystemParameter for Res<'res, T> {
    type Item<'new> = Res<'new, T>;

    fn retrieve<'r>(components: &'r Components) -> <Res<'res, T> as SystemParameter>::Item<'r> {
        Res {
            value: components.get(&(EntityId::new(0), TypeId::of::<T>())).expect("Resource is missing").borrow(),
            _marker: PhantomData,
        }
    }
}
impl<T: 'static> Deref for Res<'_, T> {
    type Target = T;

    fn deref(&self) -> &T {
        self.value.downcast_ref().unwrap()
    }
}



pub struct ResMut<'a, T: 'static> {
    value: RefMut<'a, Box<dyn Any>>,
    _marker: PhantomData<&'a mut T>,
}
impl<'res, T: 'static> SystemParameter for ResMut<'res, T> {
    type Item<'new> = ResMut<'new, T>;

    fn retrieve<'r>(components: &'r Components) -> <ResMut<'res, T> as SystemParameter>::Item<'r> {
        ResMut {
            value: components.get(&(EntityId::new(0), TypeId::of::<T>())).expect("Resource is missing").borrow_mut(),
            _marker: PhantomData,
        }
    }
}
impl<T: 'static> Deref for ResMut<'_, T> {
    type Target = T;

    fn deref(&self) -> &T {
        self.value.downcast_ref().unwrap()
    }
}
impl<T: 'static> DerefMut for ResMut<'_, T> {
    fn deref_mut(&mut self) -> &mut T {
        self.value.downcast_mut().unwrap()
    }
}