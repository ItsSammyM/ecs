use std::marker::PhantomData;
use crate::ecs::{component::components::Components, system_param::system_param::SystemParameter};


pub trait ConditionSystemContainerTrait {
    fn run(&mut self, components: &mut Components) -> bool;
}
pub struct ConditionSystemContainer<Input, F> {
    f: F,
    marker: PhantomData<fn() -> Input>,
}

pub trait IntoConditionSystem<Input> {
    type System: ConditionSystemContainerTrait;

    fn into_condition(self) -> Self::System;
}

macro_rules! system_impl {
    ($($params:ident),*) => {
        #[allow(unused_variables)]
        #[allow(non_snake_case)]
        impl<F, $($params: SystemParameter),*> IntoConditionSystem<($($params,)*)> for F 
        where
            for<'a, 'b> &'a mut F: 
                FnMut($($params),*) -> bool + 
                FnMut($(<$params as SystemParameter>::Item<'b>),*) -> bool
        {
            type System = ConditionSystemContainer<($($params,)*), Self>;

            fn into_condition(self) -> Self::System {
                ConditionSystemContainer {
                    f: self,
                    marker: Default::default(),
                }
            }
        }

        #[allow(unused_variables)]
        #[allow(non_snake_case)]
        impl<F, $($params: SystemParameter),*> ConditionSystemContainerTrait for ConditionSystemContainer<($($params,)*), F> 
        where
            // for any two arbitrary lifetimes, a mutable reference to F with lifetime 'a
            // implements FnMut taking parameters of lifetime 'b
            for<'a, 'b> &'a mut F:
                FnMut($($params),*) -> bool +
                FnMut($(<$params as SystemParameter>::Item<'b>),*) -> bool
        {
            fn run(&mut self, resources: &mut Components) -> bool{
                // necessary to tell rust exactly which impl to call; it gets a bit confused otherwise
                fn call_inner<$($params),*>(
                    
                    mut f: impl FnMut($($params ,)*) -> bool,
                    $(
                        $params: $params
                    ),*
                ) -> bool {
                    f($($params),*)
                }

                $(
                    let $params = <$params>::retrieve(resources);
                )*

                call_inner(&mut self.f, $($params),*)
            }
        }
    };
}


system_impl!();
system_impl!(T1);
system_impl!(T1, T2);
system_impl!(T1, T2, T3);
system_impl!(T1, T2, T3, T4);
system_impl!(T1, T2, T3, T4, T5);
system_impl!(T1, T2, T3, T4, T5, T6);
system_impl!(T1, T2, T3, T4, T5, T6, T7);
system_impl!(T1, T2, T3, T4, T5, T6, T7, T8);
system_impl!(T1, T2, T3, T4, T5, T6, T7, T8, T9);
system_impl!(T1, T2, T3, T4, T5, T6, T7, T8, T9, T10);
system_impl!(T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11);
system_impl!(T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12);
system_impl!(T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13);
system_impl!(T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14);
system_impl!(T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15);
system_impl!(T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16);