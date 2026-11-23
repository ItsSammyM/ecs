use crate::prelude::*;
pub trait Bundle{
    fn add_bundle<'a>(self, builder: EntitySpawnBuilder<'a>)->EntitySpawnBuilder<'a>;
}
macro_rules! bundle_tuple_impl {
    ($($param:ident),*) => {
        #[allow(unused_variables)]
        #[allow(non_snake_case)]
        impl<$($param),*> Bundle for ($($param,)*)
            where 
            $(
                $param: Component + 'static,
            )*
        {
            fn add_bundle<'a>(self, builder: EntitySpawnBuilder<'a>)->EntitySpawnBuilder<'a> {
                let ($($param,)*) = self;
                builder
                    $(
                        .add_component($param)
                    )*
            }
        } 
    };
}
bundle_tuple_impl!();
bundle_tuple_impl!(T1);
bundle_tuple_impl!(T1, T2);
bundle_tuple_impl!(T1, T2, T3);
bundle_tuple_impl!(T1, T2, T3, T4);
bundle_tuple_impl!(T1, T2, T3, T4, T5);
bundle_tuple_impl!(T1, T2, T3, T4, T5, T6);
bundle_tuple_impl!(T1, T2, T3, T4, T5, T6, T7);
bundle_tuple_impl!(T1, T2, T3, T4, T5, T6, T7, T8);
bundle_tuple_impl!(T1, T2, T3, T4, T5, T6, T7, T8, T9);
bundle_tuple_impl!(T1, T2, T3, T4, T5, T6, T7, T8, T9, T10);
bundle_tuple_impl!(T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11);
bundle_tuple_impl!(T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12);
bundle_tuple_impl!(T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13);
bundle_tuple_impl!(T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14);
bundle_tuple_impl!(T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15);
bundle_tuple_impl!(T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16);