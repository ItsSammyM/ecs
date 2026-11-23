use std::marker::PhantomData;
use crate::ecs::{component::components::Components, system_param::system_param::SystemParameter};

pub trait SystemContainerTrait {
    fn run(&mut self, components: &mut Components);
}
pub struct SystemContainer<Input, F> {
    f: F,
    marker: PhantomData<fn() -> Input>,
}

pub trait IntoSystem<Input> {
    type System: SystemContainerTrait;

    fn into_system(self) -> Self::System;
}

macro_rules! system_impl {
    ($($params:ident),*) => {
        #[allow(unused_variables)]
        #[allow(non_snake_case)]
        impl<F, $($params: SystemParameter),*> IntoSystem<($($params,)*)> for F 
        where
            for<'a, 'b> &'a mut F: 
                FnMut($($params),*) + 
                FnMut($(<$params as SystemParameter>::Item<'b>),*)
        {
            type System = SystemContainer<($($params,)*), Self>;

            fn into_system(self) -> Self::System {
                SystemContainer {
                    f: self,
                    marker: Default::default(),
                }
            }
        }

        #[allow(unused_variables)]
        #[allow(non_snake_case)]
        impl<F, $($params: SystemParameter),*> SystemContainerTrait for SystemContainer<($($params,)*), F> 
        where
            // for any two arbitrary lifetimes, a mutable reference to F with lifetime 'a
            // implements FnMut taking parameters of lifetime 'b
            for<'a, 'b> &'a mut F:
                FnMut($($params),*) +
                FnMut($(<$params as SystemParameter>::Item<'b>),*)
        {
            fn run(&mut self, resources: &mut Components) {
                // necessary to tell rust exactly which impl to call; it gets a bit confused otherwise
                fn call_inner<$($params),*>(
                    
                    mut f: impl FnMut($($params ,)*),
                    $(
                        $params: $params
                    ),*
                ) {
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