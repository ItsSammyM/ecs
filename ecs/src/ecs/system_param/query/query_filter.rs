use crate::{ecs::{component::components::Components, system_param::query::query_parameter::QueryParamter}, prelude::EntityId};

pub trait QueryFilter {
    fn filter(entity: EntityId, components: &Components) -> bool;
}
impl QueryFilter for () {
    fn filter(_entity: EntityId, _components: &Components) -> bool {
        true
    }
}

pub struct Without<T: QueryParamter> {
    _marker: std::marker::PhantomData<T>,
}
impl<T: QueryParamter> QueryFilter for Without<T> {
    fn filter(entity: EntityId, components: &Components) -> bool {
        !T::exists(entity, components)
    }
}

pub struct With<T: QueryParamter> {
    _marker: std::marker::PhantomData<T>,
}
impl<T: QueryParamter> QueryFilter for With<T> {
    fn filter(entity: EntityId, components: &Components) -> bool {
        T::exists(entity, components)
    }
}


impl<A: QueryFilter, B: QueryFilter> QueryFilter for (A, B) {
    fn filter(entity: EntityId, components: &Components) -> bool {
        A::filter(entity, components) && B::filter(entity, components)
    }
}

impl<A: QueryFilter, B: QueryFilter, C: QueryFilter> QueryFilter for (A, B, C) {
    fn filter(entity: EntityId, components: &Components) -> bool {
        A::filter(entity, components) && B::filter(entity, components) && C::filter(entity, components)
    }
}