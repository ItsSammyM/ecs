use std::cell::Ref;
use std::cell::RefMut;
use crate::prelude::With;
use crate::ecs::system_param::query::query_filter::QueryFilter;
use crate::ecs::{component::{component::Component, components::Components}, entity::EntityId};



pub trait QueryParamter {
    type Item<'a>;
    fn exists(entity: EntityId, components: &Components) -> bool;
    fn retrieve<'a>(entity: EntityId, components: &'a Components) -> Option<Self::Item<'a>>;
}
impl<T: Component + 'static> QueryParamter for &mut T {
    type Item<'a> = RefMut<'a, T>;
    fn exists(entity: EntityId, components: &Components) -> bool {
        T::get_boxed_any(entity, components).is_some()
    }
    fn retrieve<'a>(entity: EntityId, components: &'a Components) -> Option<Self::Item<'a>> {
        T::get_boxed_any(entity, components)
            .map(|r| RefMut::map(r.borrow_mut(), |r|r.downcast_mut().unwrap()))
    }
}
impl<T: Component + 'static> QueryParamter for &T {
    type Item<'a> = Ref<'a, T>;
    fn exists(entity: EntityId, components: &Components) -> bool {
        T::get_boxed_any(entity, components).is_some()
    }
    fn retrieve<'a>(entity: EntityId, components: &'a Components) -> Option<Self::Item<'a>> {
        T::get_boxed_any(entity, components)
            .map(|r| Ref::map(r.borrow(), |r|r.downcast_ref().unwrap()))
    }
}

macro_rules! query_parameter_impl {
    ($($param:ident),*) => {
        #[allow(unused_variables)]
        #[allow(non_snake_case)]
        impl<$($param: QueryParamter),*> QueryParamter for ($($param,)*)
        {
            type Item<'a> = ($($param::Item<'a>,)*);

            fn exists(entity: EntityId, components: &Components) -> bool {
                $(
                    $param::exists(entity, components)
                )&&*
            }
            fn retrieve<'a>(entity: EntityId, components: &'a Components) -> Option<Self::Item<'a>> {
                if
                    // Ensure all components exist
                    // If only some components are missing then it will still collect the ones that do exist, this can cause double borrow issues.
                    $(
                        With::<$param>::filter(entity, components)
                    )&&*
                    &&
                    $(
                        let Some($param) = $param::retrieve(entity, components)
                    )&&*
                {
                    Some(($($param,)*))
                } else {
                    None
                }
            }
        }
    }
}

query_parameter_impl!(T1);
query_parameter_impl!(T1, T2);
query_parameter_impl!(T1, T2, T3);
query_parameter_impl!(T1, T2, T3, T4);
query_parameter_impl!(T1, T2, T3, T4, T5);
query_parameter_impl!(T1, T2, T3, T4, T5, T6);
query_parameter_impl!(T1, T2, T3, T4, T5, T6, T7);
query_parameter_impl!(T1, T2, T3, T4, T5, T6, T7, T8);
query_parameter_impl!(T1, T2, T3, T4, T5, T6, T7, T8, T9);
query_parameter_impl!(T1, T2, T3, T4, T5, T6, T7, T8, T9, T10);
query_parameter_impl!(T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11);
query_parameter_impl!(T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12);
query_parameter_impl!(T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13);
query_parameter_impl!(T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14);
query_parameter_impl!(T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15);
query_parameter_impl!(T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16);